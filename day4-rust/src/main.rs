#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::OutputPin;
use heapless::String;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{Adc, Clock};
use rp_pico::{hal, hal::Sio, pac, Pins};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;
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

    let mut red_pin = pins.gpio20.into_push_pull_output();
    let mut amber_pin = pins.gpio19.into_push_pull_output();
    let mut green_pin = pins.gpio18.into_push_pull_output();

    let mut adc = Adc::new(peripherals.ADC, &mut peripherals.RESETS);

    let mut potentiometer = pins.gpio27.into_floating_input();

    loop {
        // Prevent windows to unrecogniize the device
        if timer.get_counter() >= 2_000_000 {
            let pin_adc_counts: u16 = adc.read(&mut potentiometer).unwrap();
            if pin_adc_counts <= 1500 {
                red_pin.set_high().unwrap();
                amber_pin.set_low().unwrap();
                green_pin.set_low().unwrap();
            } else if pin_adc_counts < 3500 && pin_adc_counts > 1500 {
                red_pin.set_low().unwrap();
                amber_pin.set_high().unwrap();
                green_pin.set_low().unwrap();
            } else if pin_adc_counts >= 3500 {
                red_pin.set_low().unwrap();
                amber_pin.set_low().unwrap();
                green_pin.set_high().unwrap();
            }
            let mut text: String<64> = String::new();
            writeln!(text, "ADC counts: {}\r\n", pin_adc_counts).unwrap();
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
