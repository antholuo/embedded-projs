#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::{delay::Delay, rmt::Rmt, time::Rate};
use esp_hal::time::{Duration, Instant};
use esp_hal::timer::timg::TimerGroup;
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use smart_leds::{
    RGB8, SmartLedsWrite, brightness,
    gamma,
    hsv::{Hsv, hsv2rgb},
};
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();



#[main]
fn main() -> ! {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    
    esp_alloc::heap_allocator!(size: 64 * 1024);

    // let timg0 = TimerGroup::new(peripherals.TIMG0);
    // let _init = esp_wifi::init(timg0.timer0, esp_hal::rng::Rng::new(peripherals.RNG)).unwrap();

    let frequency: Rate = Rate::from_mhz(80);
    // Configure RMT Peripheral
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        Rmt::new(peripherals.RMT, frequency)
    }.expect("Failed to initialize RMT");

    let rmt_channel = rmt.channel0;
    let rmt_buffer = smart_led_buffer!(1);
    let mut led: SmartLedsAdapter<_, 25> = { SmartLedsAdapter::new(rmt_channel, peripherals.GPIO8, rmt_buffer) };

    let delay = Delay::new();
    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data: RGB8;
    let level = 100;

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

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
