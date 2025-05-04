#![no_std]
#![no_main]

mod filesystem;
mod psram;

use core::panic::PanicInfo;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_hal_bus::spi::ExclusiveDevice;
use rp235x_hal::{
    self as hal, Clock,
    clocks::StoppableClock,
    dma::{DMAExt, double_buffer},
    fugit::RateExtU32,
    gpio::{FunctionI2C, FunctionPio0, Pin},
    pio::{PIOExt, PinDir},
};

const WIDTH: u16 = 1312;
const HEIGHT: u16 = 1048;

const U32_BUFFER_LENGTH: usize = 4;
const NUMBER_OF_BUFFER_SWAPS: usize = (WIDTH as usize * HEIGHT as usize) / U32_BUFFER_LENGTH / 4;

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
    let pins = hal::gpio::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let mut status_led = pins.gpio25.into_push_pull_output();

    let mut sensor_trigger = pins.gpio12.into_push_pull_output();

    // PSRAM setup
    let _cs = pins.gpio47.into_function::<hal::gpio::FunctionXipCs1>();
    let (_, kgd, _) = psram::read_id(&p.QMI);
    if kgd != 0x5D {
        panic!();
    }
    psram::init(&p.QMI, &mut timer, clocks.system_clock.freq().to_Hz());
    p.XIP_CTRL.ctrl().modify(|_, w| w.writable_m1().set_bit());
    let psram_memory_base = unsafe {
        core::slice::from_raw_parts_mut(psram::BASE_ADDRESS as *mut u32, 1024 * 1024 * 8 / 4)
    };

    // Sensor setup
    let _sensor_system_clk = pins.gpio21.into_function::<hal::gpio::FunctionClock>();
    clocks
        .gpio_output0_clock
        .configure_clock(&clocks.system_clock, 4.MHz())
        .unwrap();
    clocks.gpio_output0_clock.enable();
    let sensor_i2c_sda: hal::gpio::Pin<_, FunctionI2C, _> = pins.gpio14.reconfigure();
    let sensor_i2c_scl: hal::gpio::Pin<_, FunctionI2C, _> = pins.gpio15.reconfigure();
    let sensor_i2c = hal::I2C::i2c1(
        p.I2C1,
        sensor_i2c_sda,
        sensor_i2c_scl,
        100.kHz(),
        &mut p.RESETS,
        &clocks.system_clock,
    );
    let mut sensor = mt9m001::MT9M001::new(sensor_i2c);
    if sensor.get_chip_version().unwrap() != 0x8431 {
        panic!();
    }
    // Configure sensor...
    {
        sensor.set_reset(true).unwrap();
        sensor.set_reset(false).unwrap();

        sensor.set_column_start(0).unwrap();
        sensor.set_column_size(WIDTH).unwrap();
        sensor.set_row_start(0).unwrap();
        sensor.set_row_size(HEIGHT).unwrap();
        sensor.set_horizontal_blanking(0).unwrap();
        sensor.set_vertical_blanking(0).unwrap();

        sensor.set_read_options_1(0b100000000).unwrap();
        //sensor.set_output_control(false, true, true).unwrap();
    }

    let pio_capture = pio_proc::pio_file!(
        "src/main.pio",
        select_program("capture"),
        options(max_program_size = 32)
    );
    let (mut pio, sm0, _, _, _) = p.PIO0.split(&mut p.RESETS);
    let installed_program = pio.install(&pio_capture.program).unwrap();
    let sensor_d0: Pin<_, FunctionPio0, _> = pins.gpio0.into_function();
    let sensor_d1: Pin<_, FunctionPio0, _> = pins.gpio1.into_function();
    let sensor_d2: Pin<_, FunctionPio0, _> = pins.gpio2.into_function();
    let sensor_d3: Pin<_, FunctionPio0, _> = pins.gpio3.into_function();
    let sensor_d4: Pin<_, FunctionPio0, _> = pins.gpio4.into_function();
    let sensor_d5: Pin<_, FunctionPio0, _> = pins.gpio5.into_function();
    let sensor_d6: Pin<_, FunctionPio0, _> = pins.gpio6.into_function();
    let sensor_d7: Pin<_, FunctionPio0, _> = pins.gpio7.into_function();
    let sensor_pixel_clock: Pin<_, FunctionPio0, _> = pins.gpio8.into_function();
    let sensor_line_valid: Pin<_, FunctionPio0, _> = pins.gpio9.into_function();
    let (mut sm, rx, _) = hal::pio::PIOBuilder::from_installed_program(installed_program)
        .in_pin_base(sensor_d0.id().num)
        .in_count(10)
        .clock_divisor_fixed_point(1, 0)
        .autopush(true)
        .build(sm0);
    sm.set_pindirs([
        (sensor_d0.id().num, PinDir::Input),
        (sensor_d1.id().num, PinDir::Input),
        (sensor_d2.id().num, PinDir::Input),
        (sensor_d3.id().num, PinDir::Input),
        (sensor_d4.id().num, PinDir::Input),
        (sensor_d5.id().num, PinDir::Input),
        (sensor_d6.id().num, PinDir::Input),
        (sensor_d7.id().num, PinDir::Input),
        (sensor_pixel_clock.id().num, PinDir::Input),
        (sensor_line_valid.id().num, PinDir::Input),
    ]);

    sm.start();

    let dma = p.DMA.split(&mut p.RESETS);

    let rx_buf0 =
        rp235x_hal::singleton!(: [u32; U32_BUFFER_LENGTH] = [0; U32_BUFFER_LENGTH]).unwrap();
    let rx_buf1 =
        rp235x_hal::singleton!(: [u32; U32_BUFFER_LENGTH] = [0; U32_BUFFER_LENGTH]).unwrap();
    let rx_transfer = double_buffer::Config::new((dma.ch1, dma.ch2), rx, rx_buf0).start();
    let mut rx_transfer = rx_transfer.write_next(rx_buf1);

    // Trigger...
    sensor_trigger.set_high().unwrap();
    timer.delay_ms(10);
    sensor_trigger.set_low().unwrap();

    for i in 0..NUMBER_OF_BUFFER_SWAPS {
        let (rx_buf, next_rx_transfer) = rx_transfer.wait();
        for j in 0..U32_BUFFER_LENGTH {
            psram_memory_base[i * U32_BUFFER_LENGTH + j] = rx_buf[j];
        }
        rx_transfer = next_rx_transfer.write_next(rx_buf);
    }

    for _ in 0..3 {
        status_led.set_high().unwrap();
        timer.delay_ms(50);
        status_led.set_low().unwrap();
        timer.delay_ms(50);
    }

    // SDMMC and file system setup
    let sdmmc_spi_rx = pins.gpio16.into_function::<hal::gpio::FunctionSpi>();
    let sdmmc_spi_cp = pins.gpio17.into_push_pull_output();
    let sdmmc_spi_sclk = pins.gpio18.into_function::<hal::gpio::FunctionSpi>();
    let sdmmc_spi_tx = pins.gpio19.into_function::<hal::gpio::FunctionSpi>();
    let sdmmc_spi_bus =
        hal::spi::Spi::<_, _, _, 8>::new(p.SPI0, (sdmmc_spi_tx, sdmmc_spi_rx, sdmmc_spi_sclk));
    let sdmmc_spi_bus = sdmmc_spi_bus.init(
        &mut p.RESETS,
        clocks.peripheral_clock.freq(),
        24.MHz(),
        embedded_hal::spi::MODE_0,
    );
    let device = ExclusiveDevice::new_no_delay(sdmmc_spi_bus, sdmmc_spi_cp).unwrap();
    let mut card = filesystem::FileSystem::new(device, &mut timer);
    let (_, frame, _) = unsafe { psram_memory_base.align_to::<u8>() };
    card.write_new_image(&frame[0..WIDTH as usize * HEIGHT as usize]);
    drop(card);

    status_led.set_high().unwrap();

    loop {}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
