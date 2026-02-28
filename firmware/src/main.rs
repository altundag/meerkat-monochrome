#![no_std]
#![no_main]

mod fram;
mod psram;
mod sdmmc;
mod sensor;

use core::panic::PanicInfo;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_hal_bus::spi::ExclusiveDevice;
use rp235x_hal::{
    self as hal, Clock, Timer,
    clocks::StoppableClock,
    dma::{DMAExt, single_buffer},
    fugit::RateExtU32,
    gpio::{self, FunctionI2C, PinState},
    pio::PIOExt,
    timer::CopyableTimer0,
};

const U32_IMAGE_BUFFER_LENGTH: usize =
    ((sensor::WIDTH as usize + 2) * sensor::HEIGHT as usize).div_ceil(3);

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    let mut p = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(p.WATCHDOG);
    let mut clocks = hal::clocks::init_clocks_and_plls(
        12_000_000u32,
        p.XOSC,
        p.CLOCKS,
        p.PLL_SYS,
        p.PLL_USB,
        &mut p.RESETS,
        &mut watchdog,
    )
    .unwrap();
    let mut timer = hal::Timer::new_timer0(p.TIMER0, &mut p.RESETS, &clocks);
    let sio = hal::Sio::new(p.SIO);
    let pins = gpio::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let mut status_led = pins.gpio20.into_push_pull_output();

    // PSRAM setup
    // tPU >= 150us...
    timer.delay_us(300);
    let _cs = pins.gpio0.into_function::<gpio::FunctionXipCs1>();
    let (_, kgd, _) = psram::read_id(&p.QMI);
    if kgd != 0x5D {
        blink(&mut timer, &mut status_led, 3);
        panic!("cannot init psram");
    }
    psram::init(&p.QMI, &mut timer, clocks.system_clock.freq().to_Hz());
    // Make PSRAM writable
    p.XIP_CTRL.ctrl().modify(|_, w| w.writable_m1().set_bit());
    let psram_base =
        unsafe { core::slice::from_raw_parts_mut(psram::BASE_ADDRESS as *mut u8, 1024 * 1024 * 8) };

    // FRAM
    let mut fram_cs = pins.gpio17.into_push_pull_output();
    fram_cs.set_high();
    let fram_spi_rx = pins.gpio16.into_function::<gpio::FunctionSpi>();
    let fram_spi_sclk = pins.gpio18.into_function::<gpio::FunctionSpi>();
    let fram_spi_tx = pins.gpio19.into_function::<gpio::FunctionSpi>();
    let fram_spi =
        hal::spi::Spi::<_, _, _, 8>::new(p.SPI0, (fram_spi_tx, fram_spi_rx, fram_spi_sclk));
    let fram_spi = fram_spi.init(
        &mut p.RESETS,
        clocks.peripheral_clock.freq(),
        24.MHz(),
        embedded_hal::spi::MODE_0,
    );
    let mut fram = fram::FM25L16B::new(fram_cs, fram_spi);

    // Sensor
    let sensor_standby = pins.gpio4.into_push_pull_output_in_state(PinState::Low);
    let sensor_trigger = pins.gpio1.into_push_pull_output_in_state(PinState::Low);
    let _sensor_clock = pins.gpio21.into_function::<gpio::FunctionClock>();
    clocks
        .gpio_output0_clock
        .configure_clock(&clocks.system_clock, sensor::FREQUENCY.Hz())
        .unwrap();
    clocks.gpio_output0_clock.enable();
    let sensor_i2c_sda: gpio::Pin<_, FunctionI2C, _> = pins.gpio2.reconfigure();
    let sensor_i2c_scl: gpio::Pin<_, FunctionI2C, _> = pins.gpio3.reconfigure();
    let sensor_i2c = hal::I2C::i2c1(
        p.I2C1,
        sensor_i2c_sda,
        sensor_i2c_scl,
        100.kHz(),
        &mut p.RESETS,
        &clocks.system_clock,
    );
    let mut sensor = sensor::Sensor::new(
        clocks.gpio_output0_clock,
        timer,
        sensor_i2c,
        sensor_standby,
        sensor_trigger,
    );
    if sensor.init().is_err() {
        blink(&mut timer, &mut status_led, 5);
        panic!("cannot initialize the sensor");
    }

    // Sensor PIO
    let pio_capture = pio::pio_file!(
        "src/main.pio",
        select_program("capture"),
        options(max_program_size = 32)
    );
    let (mut pio, sm0, _, _, _) = p.PIO0.split(&mut p.RESETS);
    let installed_program = pio.install(&pio_capture.program).unwrap();
    let sensor_d0: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio6.into_function();
    let sensor_d1: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio7.into_function();
    let sensor_d2: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio8.into_function();
    let sensor_d3: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio9.into_function();
    let sensor_d4: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio10.into_function();
    let sensor_d5: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio11.into_function();
    let sensor_d6: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio12.into_function();
    let sensor_d7: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio13.into_function();
    let sensor_d8: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio14.into_function();
    let sensor_d9: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio15.into_function();
    let sensor_pixel_valid: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio5.into_function();
    let (mut sm, rx, _) = hal::pio::PIOBuilder::from_installed_program(installed_program)
        .in_pin_base(sensor_d0.id().num)
        .in_count(10)
        .clock_divisor_fixed_point(1, 0)
        .buffers(rp235x_hal::pio::Buffers::OnlyRx)
        .push_threshold(64)
        .autopush(true)
        .build(sm0);
    sm.set_pindirs([
        (sensor_d0.id().num, hal::pio::PinDir::Input),
        (sensor_d1.id().num, hal::pio::PinDir::Input),
        (sensor_d2.id().num, hal::pio::PinDir::Input),
        (sensor_d3.id().num, hal::pio::PinDir::Input),
        (sensor_d4.id().num, hal::pio::PinDir::Input),
        (sensor_d5.id().num, hal::pio::PinDir::Input),
        (sensor_d6.id().num, hal::pio::PinDir::Input),
        (sensor_d7.id().num, hal::pio::PinDir::Input),
        (sensor_d8.id().num, hal::pio::PinDir::Input),
        (sensor_d9.id().num, hal::pio::PinDir::Input),
        (sensor_pixel_valid.id().num, hal::pio::PinDir::Input),
    ]);

    // Sensor to PSRAM transfer (DMA)
    let (_, u32_slice, _) = unsafe { psram_base.align_to_mut::<u32>() };
    let image_buf: &mut [u32] = &mut u32_slice[..U32_IMAGE_BUFFER_LENGTH];
    let dma = p.DMA.split(&mut p.RESETS);
    let mut transfer = single_buffer::Config::new(dma.ch1, rx, image_buf);

    // SDMMC and file system setup
    let sdmmc_spi_rx = pins.gpio24.into_function::<hal::gpio::FunctionSpi>();
    let sdmmc_spi_cs = pins.gpio25.into_push_pull_output();
    let sdmmc_spi_sclk = pins.gpio26.into_function::<hal::gpio::FunctionSpi>();
    let sdmmc_spi_tx = pins.gpio27.into_function::<hal::gpio::FunctionSpi>();
    let sdmmc_spi_bus =
        hal::spi::Spi::<_, _, _, 8>::new(p.SPI1, (sdmmc_spi_tx, sdmmc_spi_rx, sdmmc_spi_sclk));
    let sdmmc_spi_bus = sdmmc_spi_bus.init(
        &mut p.RESETS,
        clocks.peripheral_clock.freq(),
        24.MHz(),
        embedded_hal::spi::MODE_0,
    );
    let sdmmc_spi_bus = ExclusiveDevice::new_no_delay(sdmmc_spi_bus, sdmmc_spi_cs)
        .expect("Failed to create SPI device");

    let mut sdmmc_memory = sdmmc::Sdmmc::new(sdmmc_spi_bus, &mut timer);

    // Capture frame...
    status_led.set_high().unwrap();
    for i in 0..10 {
        sm.clear_fifos();
        let running_sm = sm.start();
        let running_transfer = transfer.start();

        let capture_result = sensor.configure_and_capture(1f32, (1, 15 * (i + 1)), || {
            let (channel, from, to) = running_transfer.wait();
            let stopped_sm = running_sm.stop();
            (stopped_sm, (channel, from, to))
        });

        (sm, transfer) = if let Ok((returned_sm, (channel, from, to))) = capture_result {
            let image_counter = if let Ok(v) = fram.read(0).and_then(|v: u64| {
                fram.write(0, v.wrapping_add(1))?;
                Ok(v)
            }) {
                v
            } else {
                blink(&mut timer, &mut status_led, 4);
                panic!("cannot read or incrament image counter");
            };
            let image_counter = (image_counter % u16::MAX as u64) as u16;

            let (_, image, _) = unsafe { to.align_to_mut::<u8>() };
            if sdmmc_memory.write_image(image_counter, image).is_err() {
                blink(&mut timer, &mut status_led, 6);
                panic!("cannot save image");
            }

            (returned_sm, single_buffer::Config::new(channel, from, to))
        } else {
            blink(&mut timer, &mut status_led, 7);
            panic!("cannot capture frame");
        };
    }

    loop {
        let _ = status_led.set_high();
        timer.delay_ms(50);
        let _ = status_led.set_low();
        timer.delay_ms(50);
    }
}

fn blink<OP: OutputPin>(timer: &mut Timer<CopyableTimer0>, led: &mut OP, n: u8) {
    for _ in 0..n {
        let _ = led.set_high();
        timer.delay_ms(400);
        let _ = led.set_low();
        timer.delay_ms(400);
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
