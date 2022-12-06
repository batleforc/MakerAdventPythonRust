#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::prelude::*;
use embedded_hal::PwmPin;
use fugit::ExtU32;
use heapless::String;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::pwm::{Channel, FreeRunning, Pwm6, Slice, B};
use rp_pico::hal::timer::CountDown;
use rp_pico::hal::{Adc, Clock};
use rp_pico::{hal, hal::Sio, pac, Pins};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 25000;

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

    let sio = Sio::new(peripherals.SIO);

    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        peripherals.USBCTRL_REGS,
        peripherals.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut peripherals.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let timer = hal::Timer::new(peripherals.TIMER, &mut peripherals.RESETS);
    let mut count_down = &mut timer.count_down();

    let mut delay = &mut cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut pwm_slices = hal::pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

    let pwm6 = &mut pwm_slices.pwm6;
    pwm6.set_ph_correct();
    pwm6.enable();

    pub fn set_pwm_frequency(
        pwm: &mut Slice<Pwm6, FreeRunning>,
        freq: u16,
    ) -> &mut Channel<Pwm6, FreeRunning, B> {
        pwm.set_ph_correct();
        pwm.set_top(freq);
        pwm.enable();
        pwm.channel_b.set_duty(freq / 2);
        &mut pwm.channel_b
    }
    fn calc_note(freq: f32) -> u16 {
        (12_000_000 as f32 / 40 as f32 / freq) as u16
    }
    let C: u16 = calc_note(523.0);
    let D: u16 = calc_note(587.0);
    let E: u16 = calc_note(659.0);
    let G: u16 = calc_note(784.0);

    let volume: u16 = 32768;

    let mut buzzer = set_pwm_frequency(pwm6, 40u16);

    pub fn playtone<'a>(
        note: u16,
        volume: u16,
        delay1: u32,
        delay2: u32,
        chan: &'a mut Channel<Pwm6, FreeRunning, B>,
        countDown: &'a mut CountDown<'a>,
    ) -> (&'a mut Channel<Pwm6, FreeRunning, B>, &'a mut CountDown<'a>) {
        chan.set_duty(volume * note);
        countDown.start(400.millis());
        let _ = nb::block!(countDown.wait());
        chan.set_duty(0);
        countDown.start(400.millis());
        let _ = nb::block!(countDown.wait());

        return (chan, countDown);
    }

    loop {
        // Prevent windows to unrecognize the device
        if timer.get_counter() >= 2_000_000 {
            let (mut buzzer_chan_swap, mut delay_swap) =
                playtone(E, volume, 0.1 as u32, 0.2 as u32, buzzer, count_down);
            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.5 as u32,
                buzzer_chan_swap,
                delay_swap,
            );

            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.5 as u32,
                buzzer_chan_swap,
                delay_swap,
            );

            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                G,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                C,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                D,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );
            (buzzer_chan_swap, delay_swap) = playtone(
                E,
                volume,
                0.1 as u32,
                0.2 as u32,
                buzzer_chan_swap,
                delay_swap,
            );

            count_down = delay_swap;
            buzzer = buzzer_chan_swap;

            let mut text: String<64> = String::new();
            writeln!(text, "Hello World \r\n").unwrap();
            let _ = serial.write(text.as_bytes());
            delay.delay_ms(250);
        }

        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    // Convert to upper case
                    buf.iter_mut().take(count).for_each(|b| {
                        b.make_ascii_uppercase();
                    });
                    // Send back to the host
                    let mut wr_ptr = &buf[..count];
                    while !wr_ptr.is_empty() {
                        match serial.write(wr_ptr) {
                            Ok(len) => wr_ptr = &wr_ptr[len..],
                            // On error, just drop unwritten data.
                            // One possible error is Err(WouldBlock), meaning the USB
                            // write buffer is full.
                            Err(_) => break,
                        };
                    }
                }
            }
        }
    }
}

// TRYC THAT https://bandarra.me/2022/08/02/Play-Music-with-the-Raspberry-Pi-Pico-and-Rust/
