#![no_std]
#![no_main]

mod fram;
mod sdmmc;
mod sensor;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts, clocks,
    gpio::{Level, Output},
    i2c, peripherals, pio, spi,
};
use embassy_time::Timer;
use fixed::{FixedU32, traits::ToFixed, types::extra::U16};

use mt9m001::MT9M001;
use sensor::Sensor;

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Meerkat Monochrome Firmware"),
    embassy_rp::binary_info::rp_program_description!(c"Meerkat Monochrome Firmware"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<peripherals::PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = embassy_rp::init(Default::default());

    let mut status_led = Output::new(p.PIN_20, Level::Low);

    // PSRAM
    let mut psram_config = embassy_rp::psram::Config::aps6404l();
    psram_config.clock_hz = clocks::clk_sys_freq();
    let psram = embassy_rp::psram::Psram::new(
        embassy_rp::qmi_cs1::QmiCs1::new(p.QMI_CS1, p.PIN_0),
        psram_config,
    );
    let psram = if let Ok(psram) = psram {
        psram
    } else {
        blink(&mut status_led, 3).await;
        panic!("cannot initialize psram");
    };
    let psram_slice = unsafe {
        core::slice::from_raw_parts_mut(psram.base_address() as *mut u32, psram.size() / 4)
    };

    // FRAM
    let mut fram = fram::FM25L16B::new(
        Output::new(p.PIN_17, Level::High),
        spi::Spi::new_blocking(p.SPI0, p.PIN_18, p.PIN_19, p.PIN_16, spi::Config::default()),
    );

    // Sensor PIO
    let pio::Pio {
        mut common,
        sm0: mut sensor_pio_sm,
        ..
    } = pio::Pio::new(p.PIO0, Irqs);
    let sensor_pio_program = pio::program::pio_file!(
        "src/main.pio",
        select_program("capture"),
        options(max_program_size = 32)
    );
    let sensor_pio_pins = [
        &common.make_pio_pin(p.PIN_5),
        &common.make_pio_pin(p.PIN_6),
        &common.make_pio_pin(p.PIN_7),
        &common.make_pio_pin(p.PIN_8),
        &common.make_pio_pin(p.PIN_9),
        &common.make_pio_pin(p.PIN_10),
        &common.make_pio_pin(p.PIN_11),
        &common.make_pio_pin(p.PIN_12),
        &common.make_pio_pin(p.PIN_13),
        &common.make_pio_pin(p.PIN_14),
        &common.make_pio_pin(p.PIN_15),
    ];
    let mut sensor_pio_cfg = pio::Config::default();
    sensor_pio_cfg.set_in_pins(&sensor_pio_pins[1..]);
    sensor_pio_cfg.fifo_join = pio::FifoJoin::RxOnly;
    sensor_pio_cfg.clock_divider = 1.to_fixed();
    sensor_pio_cfg.use_program(&common.load_program(&sensor_pio_program.program), &[]);
    sensor_pio_cfg.shift_in.threshold = 32;
    sensor_pio_cfg.shift_in.auto_fill = true;
    sensor_pio_cfg.shift_in.direction = pio::ShiftDirection::Right;
    sensor_pio_sm.set_config(&sensor_pio_cfg);
    sensor_pio_sm.set_pin_dirs(pio::Direction::In, &sensor_pio_pins);

    // Sensor
    let sensor_clock = clocks::Gpout::new(p.PIN_21);
    sensor_clock.set_src(clocks::GpoutSrc::Sys);
    let sensor_clock_divider =
        FixedU32::<U16>::from_num(clocks::clk_sys_freq() as f32 / Sensor::FREQUENCY as f32);
    sensor_clock.set_div(
        sensor_clock_divider.int().to_num(),
        sensor_clock_divider.frac().to_bits() as u16,
    );
    let mut sensor = Sensor::new(
        sensor_clock,
        i2c::I2c::new_blocking(p.I2C1, p.PIN_3, p.PIN_2, i2c::Config::default()),
        Output::new(p.PIN_4, Level::High),
        Output::new(p.PIN_1, Level::High),
    );
    if Ok(true) != sensor.is_known_sensor() {
        blink(&mut status_led, 5).await;
        panic!("unknwon sensor");
    }
    if sensor.init().is_err() {
        blink(&mut status_led, 6).await;
        panic!("cannot initialize sensor");
    }

    // SD Card
    let spi = spi::Spi::new_blocking(p.SPI1, p.PIN_26, p.PIN_27, p.PIN_24, spi::Config::default());
    let cs = Output::new(p.PIN_25, Level::Low);
    let mut sd_card = sdmmc::Sdmmc::new(spi, cs);

    for denominator in [1, 2, 4, 5, 10, 20, 30, 60, 120, 200, 500, 1000] {
        let image_counter = if let Ok(v) = fram.read(0).and_then(|v: u64| {
            fram.write(0, v.wrapping_add(1))?;
            Ok(v)
        }) {
            v
        } else {
            blink(&mut status_led, 4).await;
            panic!("cannot read or incrament image counter");
        };
        let image_counter = (image_counter % u16::MAX as u64) as u16;

        let image = &mut psram_slice
            [..((Sensor::WIDTH as usize + 2) * Sensor::HEIGHT as usize).div_ceil(3)];

        sensor.set_gain(1f32);
        sensor.set_shutter_speed(1, denominator);

        status_led.set_high();
        // Capture
        let is_capture_failed = sensor
            .capture(&mut sensor_pio_sm, p.DMA_CH1.reborrow(), image)
            .await
            .is_err();
        if is_capture_failed {
            blink(&mut status_led, 8).await;
            panic!("cannot capture frame");
        }
        status_led.set_low();

        let (_, image, _) = unsafe { image.align_to::<u8>() };
        if sd_card.write_image(image_counter, image).is_err() {
            blink(&mut status_led, 9).await;
            panic!("cannot write image data");
        }
    }

    status_led.set_high();

    loop {}
}

async fn blink<'a>(led: &mut Output<'a>, n: u8) {
    for _ in 0..n {
        led.set_high();
        Timer::after_millis(400).await;
        led.set_low();
        Timer::after_millis(400).await;
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
