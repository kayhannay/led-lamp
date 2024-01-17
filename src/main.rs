#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay, Rmt};
use esp_backtrace as _;
use esp_println::println;
use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
use palette::{FromColor, Hsv, Srgb};
use smart_leds::RGB8;
use smart_leds::SmartLedsWrite;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let rmt = Rmt::new(peripherals.RMT, 80u32.MHz(), &clocks).unwrap();
    let rmt_buffer = smartLedBuffer!(18);
    let mut led = SmartLedsAdapter::new(rmt.channel0, io.pins.gpio0, rmt_buffer);

    let mut h: f32 = 0.0;
    let s: f32 = 1.0;
    let v: f32 = 0.8;
    let mut leds: [RGB8;18] = [RGB8::default(); 18];
    loop {
        h += 2.0;
        if h > 360.0 {
            h = 0.0;
        }

        for i in 0..18 {
            let rgb = Srgb::from_color(Hsv::new(h + (i as f32 * 20.0), s, v));
            let (r, g, b) = ((rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8);
            leds[i] = RGB8::new(r, g, b);
        }
        led.write(leds.iter().cloned()).unwrap();
        delay.delay_ms(25u32);
    }
}
