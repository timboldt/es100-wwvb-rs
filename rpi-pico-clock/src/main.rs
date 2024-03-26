//! ```text
//!                              VCC   SCL
//!                   /------------\    /----------\
//!                   |        GND  \  /  SDA      |
//!   _|USB|_         |    /-----\  |  |  /--------+--\
//!  |1  R 40|        |   /    __|__|__|__|___     |  |
//!  |2  P 39|        |  /    |   ES100-MOD   |    |  |
//!  |3    38|- GND --+-/     |               |    |  |
//!  |4  P 37|        |        """""""""""""""     |  |
//!  |5  I 36|-+3.3V -/                            |  |
//!  |6  C   |                                     |  |
//!  |7  O   |                                     |  |
//!  |       |                                     |  |
//!  |       |                                     |  |
//!  .........                                     /  /
//!  |       |                                    /  /
//!  |     22|-GP17 I2C0 SCL---------------------/  /
//!  |20   21|-GP16 I2C0 SDA-----------------------/
//!   """""""
//! Symbols:
//!     - (+) crossing lines, not connected
//!     - (o) connected lines
//! ```

#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::blocking::delay::DelayMs;
use panic_probe as _;
use rp_pico as bsp;

// Time handling traits:
use fugit::RateExtU32;

#[entry]
fn main() -> ! {
    info!("Program start");

    let mut pac = bsp::hal::pac::Peripherals::take().unwrap();
    let mut watchdog = bsp::hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let sio = bsp::hal::sio::Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = hal::clocks::init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, hal::gpio::PullUp> =
        pins.gpio16.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, hal::gpio::PullUp> =
        pins.gpio17.reconfigure();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let _i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    loop {
        timer.delay_ms(500);
    }
}
