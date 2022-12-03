#![no_std]
#![no_main]

use embedded_hal::digital::v2::{InputPin, OutputPin};
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

    let button_one = pins.gpio13.into_pull_down_input();
    let button_two = pins.gpio8.into_pull_down_input();
    let button_three = pins.gpio3.into_pull_down_input();

    loop {
        if button_one.is_high().unwrap() {
            green_pin.set_high().unwrap();
        } else {
            green_pin.set_low().unwrap();
        }
        if button_two.is_high().unwrap() {
            amber_pin.set_high().unwrap();
        } else {
            amber_pin.set_low().unwrap();
        }
        if button_three.is_high().unwrap() {
            red_pin.set_high().unwrap();
        } else {
            red_pin.set_low().unwrap();
        }
    }
}
