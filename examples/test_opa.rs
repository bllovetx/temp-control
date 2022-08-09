// ref: https://github.com/antoinevg/stm32f3-rust-examples/blob/master/src/stm32f3-02-dma.rs
#![no_std]
#![no_main]

use cortex_m::asm;
// use cortex_m_semihosting::hprintln;
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3xx_hal as hal;
use hal::{pac};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = &dp.RCC;

    // enable GPIOA and DAC clocks
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
    rcc.apb1enr.modify(|_, w| w.dac1en().set_bit());

    // configure PA04(DAC_OUT1) as analog, floating
    let gpioa = &dp.GPIOA;
    gpioa.moder.modify(|_, w| w.moder4().analog());
    gpioa.pupdr.modify(|_, w| w.pupdr4().floating());

    // configure DAC
    let dac = &dp.DAC1;
    dac.cr.write(|w| w.boff1().enabled());     // disable dac output buffer for channel 1

    // enable DAC
    dac.cr.modify(|_, w| w.en1().enabled());    // enable dac channel 1

    // let delta = 100;
    let mut index = 0;
    // hprintln!("hello world").unwrap();

    loop {
        dac.dhr12l1.write(|w| w.dacc1dhr().bits(2048));
        index = (index + 1) % 2;
        // hprintln!("{}", dac.dor1.read().dacc1dor().bits()).unwrap();
        asm::delay(1_000_000);
    }
}