#![no_std]
#![no_main]

use cortex_m::asm;
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f3xx_hal::pac;

fn setup_mco(gpioa: &mut pac::GPIOA, rcc: &mut pac::RCC) {
    // enable gpioa clock
    rcc.ahbenr.write(|w| {
        w.iopaen().enabled()
    });
    // pa8/af0
    gpioa.moder.write(|w| w.moder8().alternate());
    gpioa.otyper.write(|w| w.ot8().push_pull());
    gpioa.ospeedr.write(|w| w.ospeedr8().high_speed());
    gpioa.afrh.write(|w| w.afrh8().af0());
}

fn setup_clock_with_pll(rcc: &mut pac::RCC) {
    // let freq = 32; // MHz
    // ensure pll off
    rcc.cr.write(|w| w.pllon().off());
    // set pll src and para
    rcc.cfgr.write(|w| {
        w.pllmul().mul8()
            .pllsrc().hsi_div2()
    });
    // open pll and wait
    rcc.cr.write(|w| w.pllon().on());
    while rcc.cr.read().pllrdy().is_not_ready() {}
    // set sysclk src and wait
    rcc.cfgr.write(|w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}
}


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC;
    let mut gpioa = dp.GPIOA;

    // set mco
    setup_mco(&mut gpioa, &mut rcc);
    rcc.cfgr.write(|w| w.mco().sysclk());

    // set pll and sysclk
    setup_clock_with_pll(&mut rcc);






    

    // // # debug

    // hprintln!("{:?}", clocks).unwrap();
    // let pllenable = rcc.cr.read().pllon().is_on();
    // let pllrdy = rcc.cr.read().pllrdy().is_ready();
    // hprintln!("pll is on: {}, pll is ready: {}", pllenable, pllrdy).unwrap();

    // // print current sw status
    // let sws = rcc.cfgr.read().sws().variant();
    // let sw = rcc.cfgr.read().sw().variant();
    // hprintln!("sws: {:?}, sw: {:?}", sws, sw).unwrap();

    // rcc.cfgr.write(|w|{
    //     w.sw().pll()
    // });

    // asm::delay(100_000_000);

    
    
    // // print current sw status again
    // let sws = rcc.cfgr.read().sws().variant();
    // let sw = rcc.cfgr.read().sw().variant();
    // hprintln!("sws: {:?}, sw: {:?}", sws, sw).unwrap();

    // rcc.cfgr.write(|w| { 
    //     w
    //         .mco().sysclk()
    // });

    
    // // print current sw status again
    // let sws = rcc.cfgr.read().sws().variant();
    // let sw = rcc.cfgr.read().sw().variant();
    // hprintln!("sws: {:?}, sw: {:?}", sws, sw).unwrap();



    loop{}
}