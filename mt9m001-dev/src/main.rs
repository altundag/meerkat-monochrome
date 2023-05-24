//! # Pico Blinky Example
//!
//! Blinks the LED on a Pico board.
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for
//! the on-board LED.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use fugit::RateExtU32;

use rp_pico::hal::clocks::StoppableClock;
use rp_pico::hal::Clock;
// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
// USB Communications Class Device support
use usbd_serial::SerialPort;

// Used to demonstrate writing formatted strings
use core::fmt::Write;
use heapless::String;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let mut clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Pico SDK version:
    // clock_gpio_init(GP21, CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLK_SYS, divider);
    // Details: https://forums.raspberrypi.com/viewtopic.php?t=336927
    clocks
        .gpio_output0_clock
        .configure_clock(&clocks.system_clock, 4.MHz())
        .unwrap();
    clocks.gpio_output0_clock.enable();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    delay.delay_ms(100);

    let mut _camera_clock = pins.gpio21.into_mode::<hal::gpio::FunctionClock>();

    // Configure two pins as being I2C, not GPIO
    let sda_pin = pins.gpio4.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio5.into_mode::<hal::gpio::FunctionI2C>();

    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        100.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut camera = mt9m001::MT9M001::new(i2c);

    let chip_version = match camera.get_chip_version() {
        Ok(val) => val,
        Err(err_code) => match err_code {
            mt9m001::Error::Value => 0xFFFC,
            mt9m001::Error::I2CRead => 0xFFFD,
            mt9m001::Error::I2CWrite => 0xFFFF,
        },
    };

    loop {
        let mut text: String<64> = String::new();
        writeln!(&mut text, "Chip version: 0x{:04X}\r\n", chip_version).unwrap();

        if serial.flush().is_ok() {
            let _ = serial.write(text.as_bytes());
        }

        if usb_dev.poll(&mut [&mut serial]) {}
    }
}

// End of file
