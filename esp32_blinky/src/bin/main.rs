#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::{delay::Delay,
rmt::Rmt, time::Rate};
use esp_hal::main;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use smart_leds::{
    RGB8, SmartLedsWrite, brightness,
    gamma,
    hsv::{Hsv, hsv2rgb},
};
use log::info;

#[main]
fn main() -> ! {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let frequency: Rate = Rate::from_mhz(80);
    // Configure RMT Peripheral
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        Rmt::new(_peripherals.RMT, frequency)
    }.expect("Failed to initialize RMT");

    let rmt_channel = rmt.channel0;
    let rmt_buffer = smart_led_buffer!(1);
    let mut led: SmartLedsAdapter<_, 25> = { SmartLedsAdapter::new(rmt_channel, _peripherals.GPIO8, rmt_buffer) };

    let delay = Delay::new();
    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data: RGB8;
    let level = 10;
    loop {
        info!("Hello world!");
        for hue in 0..=255 {
            color.hue = hue;
            data = hsv2rgb(color);
            led.write(brightness(gamma([data].into_iter()), level)).unwrap();
            delay.delay_millis(20);
        }
        delay.delay_millis(500);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}
