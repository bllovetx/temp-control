#![no_std]
#![no_main]

use cortex_m::asm;
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3xx_hal as hal;
use hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    // let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    // let mut led = gpioe
    //       .pe13
    //       .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut pa1 = gpioa
          .pa1
          .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    loop {
          pa1.toggle().unwrap();
          asm::delay(100_000);
    }
}