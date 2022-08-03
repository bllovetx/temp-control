// ref: https://github.com/antoinevg/stm32f3-rust-examples/blob/master/src/stm32f3-02-dma.rs
#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3xx_hal as hal;
use hal::{
    adc,
    pac,
    prelude::*, time::rate::Megahertz
};

fn init_dac1_in1_no_trigger(dp: &pac::Peripherals) {
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
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    init_dac1_in1_no_trigger(&dp);// pa4

    let mut rcc = dp.RCC.constrain();
    // TODO: Currently using 8MHz, switch to 72MHz for better conversion rate
    let clocks = rcc.cfgr.sysclk(Megahertz(8)).freeze(&mut dp.FLASH.constrain().acr);

    // prepare adc
    let mut adc_common = adc::CommonAdc::new(dp.ADC1_2, &clocks, &mut rcc.ahb);
    let mut adc = adc::Adc::new(
        dp.ADC1,
        adc::config::Config::default(),
        &clocks,
        &adc_common,
    ).into_oneshot();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut pa1 = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    // output content
    let amp = [4000, 2000, 1000, 500, 300];
    let mut index = 0;
    // hprintln!("hello world").unwrap();

    loop {
        let cur_amp = amp[index];
        index = (index + 1) % 5;
        let dac = &dp.DAC1;
        dac.dhr12l1.write(|w| w.dacc1dhr().bits(cur_amp));
        let temp_data: u16 = adc.read(&mut pa1).unwrap();
        hprintln!("{}", temp_data).unwrap();
        asm::delay(100_000);
    }
}