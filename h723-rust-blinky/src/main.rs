//! Basic example that produces a 1Hz square-wave on Pin PE1

#![no_main]
#![no_std]

// use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[entry]
fn main() -> ! {
    // Core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();

    // Device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Try and setup power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze(); // I do not know what this line does

    // constrain and freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // grab gpio
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // configure PE1 as output
    let mut led_orange = gpioe.pe1.into_push_pull_output();
    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_red = gpiob.pb14.into_push_pull_output();

    // get delay provider
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        led_green.set_high();
        led_orange.set_low();
        led_red.set_low();
        delay.delay_ms(500_u16);

        led_green.set_high();
        led_orange.set_high();
        led_red.set_low();
        delay.delay_ms(500_u16);

        led_green.set_high();
        led_orange.set_high();
        led_red.set_high();
        delay.delay_ms(500_u32);

        led_green.set_low();
        led_orange.set_high();
        led_red.set_high();
        delay.delay_ms(500_u16);

        led_green.set_low();
        led_orange.set_low();
        led_red.set_high();
        delay.delay_ms(500_u16);

        led_green.set_low();
        led_orange.set_low();
        led_red.set_low();
        delay.delay_ms(500_u32);
    }
}