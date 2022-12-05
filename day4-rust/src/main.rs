#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_hal::adc::OneShot;
use embedded_hal::PwmPin;
use heapless::String;
use panic_halt as _;
use rp_pico::entry;
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut pwm_slices = hal::pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

    let mut adc = Adc::new(peripherals.ADC, &mut peripherals.RESETS);

    let mut potentiometer = pins.gpio27.into_floating_input();

    let pwm2 = &mut pwm_slices.pwm2;
    pwm2.set_ph_correct();
    pwm2.enable();
    let pwm1 = &mut pwm_slices.pwm1;
    pwm1.set_ph_correct();
    pwm1.enable();
    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    let red_channel = &mut pwm2.channel_a;
    red_channel.output_to(pins.gpio20);
    let green_channel = &mut pwm1.channel_a;
    green_channel.output_to(pins.gpio18);
    let amber_channel = &mut pwm1.channel_b;
    amber_channel.output_to(pins.gpio19);

    loop {
        // Prevent windows to unrecogniize the device
        if timer.get_counter() >= 2_000_000 {
            let pin_adc_counts: u16 = adc.read(&mut potentiometer).unwrap();
            if pin_adc_counts <= 1500 {
                for i in (LOW..=HIGH).skip(100) {
                    delay.delay_us(8);
                    red_channel.set_duty(i);
                }
                for i in (LOW..=HIGH).rev().skip(100) {
                    delay.delay_us(8);
                    red_channel.set_duty(i);
                }
            } else if pin_adc_counts < 3500 && pin_adc_counts > 1500 {
                for i in (LOW..=HIGH).skip(100) {
                    delay.delay_us(8);
                    amber_channel.set_duty(i);
                }
                for i in (LOW..=HIGH).rev().skip(100) {
                    delay.delay_us(8);
                    amber_channel.set_duty(i);
                }
            } else if pin_adc_counts >= 3500 {
                for i in (LOW..=HIGH).skip(100) {
                    delay.delay_us(8);
                    green_channel.set_duty(i);
                }
                for i in (LOW..=HIGH).rev().skip(100) {
                    delay.delay_us(8);
                    green_channel.set_duty(i);
                }
            }
            let mut text: String<64> = String::new();
            writeln!(text, "ADC counts: {:02}\r\n", pin_adc_counts).unwrap();
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
