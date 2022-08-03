#![no_std]
#![no_main]

use cortex_m::asm;
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

    let mut unsafe_rcc = unsafe { &*RCC::ptr() };
    // set mco
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    // pa8/AF0 afrh config 8-15pin
    let pa8:Pin<Gpioa, U<8>, Alternate<PushPull, 0>> = gpioa.pa8
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    unsafe_rcc.cfgr.write(|w| { 
        w
            .mcopre().div1()
            .mco().sysclk()
    });

    let clocks = rcc.cfgr
        .sysclk(32.MHz())// HSI support max 64MHz sysclk
        // .use_hse(8.MHz())// To enable 72MHz, should use hse
        // .bypass_hse() // However, hse can't use on current board
        .use_pll()
        .freeze(&mut dp.FLASH.constrain().acr);


    

    // # debug

    hprintln!("{:?}", clocks).unwrap();
    let pllenable = unsafe_rcc.cr.read().pllon().is_on();
    let pllrdy = unsafe_rcc.cr.read().pllrdy().is_ready();
    hprintln!("pll is on: {}, pll is ready: {}", pllenable, pllrdy).unwrap();

    // print current sw status
    let sws = unsafe_rcc.cfgr.read().sws().variant();
    let sw = unsafe_rcc.cfgr.read().sw().variant();
    hprintln!("sws: {:?}, sw: {:?}", sws, sw).unwrap();

    unsafe_rcc.cfgr.write(|w|{
        w.sw().pll()
    });


    
    
    // print current sw status again
    let sws = unsafe_rcc.cfgr.read().sws().variant();
    let sw = unsafe_rcc.cfgr.read().sw().variant();
    hprintln!("sws: {:?}, sw: {:?}", sws, sw).unwrap();

    // unsafe_rcc.cfgr.write(|w| { 
    //     w
    //         .mco().sysclk()
    // });

    
    // print current sw status again
    let sws = unsafe_rcc.cfgr.read().sws().variant();
    let sw = unsafe_rcc.cfgr.read().sw().variant();
    hprintln!("sws: {:?}, sw: {:?}", sws, sw).unwrap();



    loop{}
}