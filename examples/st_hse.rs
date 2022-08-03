#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f3xx_hal::{
    pac::{
        self, RCC
    },
    prelude::*, gpio::{Pin, Gpioa, Alternate, U, PushPull}
};


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    // let clocks = rcc.cfgr
    //     .sysclk(72.MHz())// set sysclk to 72MHz
    //     .use_hse(8.MHz())// HSI support max 64MHz sysclk, so use hse
    //     .bypass_hse()
    //     .freeze(&mut dp.FLASH.constrain().acr);

    // hprintln!("{:?}", clocks).unwrap();
    let mut unsafe_rcc = unsafe { &*RCC::ptr() };
    // let pllenable = unsafe_rcc.cr.read().pllon().is_on();
    // hprintln!("pll is on: {}", pllenable).unwrap();
    unsafe_rcc.cr.write(|w| {
        w.hsebyp().bypassed()
            .hseon().on()
    });

    while unsafe_rcc.cr.read().hserdy().is_not_ready() {}

    // set mco
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    // pa8/AF0 afrh config 8-15pin
    let pa8:Pin<Gpioa, U<8>, Alternate<PushPull, 0>> = gpioa.pa8
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    unsafe_rcc.cfgr.write(|w| { 
        w
            .mcopre().div1()
            .mco().hse()
    });

    loop{}
}