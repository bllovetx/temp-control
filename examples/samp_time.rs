#![no_std]
#![no_main]

// use cortex_m::asm;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3xx_hal as hal;
use hal::{
    adc,
    pac,
    prelude::*
};


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
        .sysclk(64.MHz())
        .freeze(&mut dp.FLASH.constrain().acr);
    // hprintln!("{:?}", clocks).unwrap();

    // prepare adc pa1(adc1_in2)
    let  unsafe_rcc = unsafe { &*pac::RCC::ptr() };
    unsafe_rcc.cfgr2.write(|w| w.adc12pres().no_clock());// ensure using ahb clock
    let adc_common = adc::CommonAdc::new(dp.ADC1_2, &clocks, &mut rcc.ahb);
    let mut adc = adc::Adc::new(
        dp.ADC1,
        adc::config::Config::default(),
        &clocks,
        &adc_common,
    );
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let pa1 = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    // config cont mode
    adc.stop_conversion();
    while adc.is_conversion_ongoing() {}
    adc.set_conversion_mode(adc::config::ConversionMode::Continuous);
    adc.set_dma_mode(adc::config::DmaMode::Disabled);
    adc.set_external_trigger(None);
    adc.disable_interrupt(adc::Event::EndOfConversion);
    adc.disable_interrupt(adc::Event::EndOfSequence);
    adc.set_sequence_length(adc::config::Sequence::One);
    adc.set_sample_time(&pa1, adc::config::SampleTime::Cycles1C5);
    let unsafe_adc1 = unsafe { &*pac::ADC1::ptr() };
    unsafe_adc1.sqr1.write(|w| unsafe{ w.sq1().bits(adc::channel::Id::Two.into()) });
    unsafe_adc1.isr.reset();

    // prepare gpio (pa2)
    let mut pa2 = gpioa
          .pa2
          .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // record cycle num
    let mut numb: u32 = 0;// modified in loop can't use shadowing

    // start adc
    // hprintln!("prepare finished!").unwrap();
    unsafe_adc1.cr.write(|w| w.adstart().start());

    loop {
        while unsafe_adc1.isr.read().eoc().is_not_complete() {}
        // update state
        unsafe_adc1.isr.reset();
        numb = numb + 1;
        // print time per 1M cycles
        if numb % 1_000 == 0 {
            // dac output
            pa2.toggle().unwrap();
        }
    }
}
