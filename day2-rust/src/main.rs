#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::Clock;
use rp_pico::{hal, hal::Sio, pac, Pins};

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = Sio::new(peripherals.SIO);
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut red_pin = pins.gpio20.into_push_pull_output();
    let mut amber_pin = pins.gpio19.into_push_pull_output();
    let mut green_pin = pins.gpio18.into_push_pull_output();

    let mut value = 0;

    loop {
        if value == 0 {
            red_pin.set_high().unwrap();
        } else {
            red_pin.set_low().unwrap();
        }
        if value == 1 {
            amber_pin.set_high().unwrap();
        } else {
            amber_pin.set_low().unwrap();
        }
        if value == 2 {
            green_pin.set_high().unwrap();
        } else {
            green_pin.set_low().unwrap();
        }
        value = (value + 1) % 3;
        delay.delay_ms(250);
    }
}
