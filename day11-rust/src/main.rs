#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_hal::adc::OneShot;
use fugit::RateExtU32;
use heapless::String;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{Adc, Clock};
use rp_pico::{hal, hal::Sio, pac, Pins};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, Ssd1306};

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

    let mut adc = Adc::new(peripherals.ADC, &mut peripherals.RESETS);

    let mut light_captor = pins.gpio26.into_floating_input();

    let sda_pin = pins.gpio0.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio1.into_mode::<hal::gpio::FunctionI2C>();
    let i2c = hal::I2C::i2c0(
        peripherals.I2C0,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut peripherals.RESETS,
        &clocks.peripheral_clock,
    );

    let interface = ssd1306::I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X12)
        .text_color(BinaryColor::On)
        .build();
    let mut buf = FmtBuf::new();
    loop {
        // Prevent windows to unrecogniize the device
        if timer.get_counter() >= 2_000_000 {
            buf.reset();
            let pin_adc_counts: f32 = adc.read(&mut light_captor).unwrap();
            let light_level = f32::from(pin_adc_counts) / 4096.0 * 100.0;
            display.clear();
            delay.delay_ms(250);

            Text::with_baseline("Hello World", Point::zero(), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();

            let mut text: String<64> = String::new();
            writeln!(text, "Light level: {:.3}%\r\n", light_level).unwrap();
            let _ = serial.write(text.as_bytes());
            writeln!(buf, "{:.3}%\r", light_level).unwrap();
            Text::with_baseline("Light level:", Point::new(0, 11), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
            Text::with_baseline(buf.as_str(), Point::new(0, 22), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();

            display.flush().unwrap();
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

struct FmtBuf {
    buf: [u8; 64],
    ptr: usize,
}

impl FmtBuf {
    fn new() -> Self {
        Self {
            buf: [0; 64],
            ptr: 0,
        }
    }

    fn reset(&mut self) {
        self.ptr = 0;
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[0..self.ptr]).unwrap()
    }
}

impl core::fmt::Write for FmtBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rest_len = self.buf.len() - self.ptr;
        let len = if rest_len < s.len() {
            rest_len
        } else {
            s.len()
        };
        self.buf[self.ptr..(self.ptr + len)].copy_from_slice(&s.as_bytes()[0..len]);
        self.ptr += len;
        Ok(())
    }
}
