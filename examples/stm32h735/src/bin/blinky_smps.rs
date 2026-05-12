#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = rcc_setup::stm32h735g_init();
    info!("Hello World!");

    // Using PH1 to demonstrate that it can be used when using a TCXO.
    // Note: when using an high-speed external oscillator (not TCXO) PH1 is unavailable for use.
    let mut led = Output::new(p.PH1, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(500).await;

        info!("low");
        led.set_low();
        Timer::after_millis(500).await;
    }
}

mod rcc_setup {

    use embassy_stm32::rcc::{Hse, HseMode, *};
    use embassy_stm32::time::Hertz;
    use embassy_stm32::{Config, Peripherals};

    pub fn stm32h735g_init() -> Peripherals {

        // setup power and clocks for an stm32h735ig using a 50Mhz TCXO using a single pin PHO.  PH1 is free for use as GPIO.
        let mut config = Config::default();
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(50),
            mode: HseMode::Bypass,
        });
        config.rcc.hsi = None;
        config.rcc.csi = false;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::Div4, // 12.5Mhz
            mul: PllMul::Mul44,     // 550Mhz
            divp: Some(PllDiv::Div1), // 550Mhz
            divq: Some(PllDiv::Div4), // 110Mhz
            divr: Some(PllDiv::Div2), // 275Mhz
        });
        config.rcc.pll2 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::Div5, // 10Mhz
            mul: PllMul::Mul40,      // 400Mhz
            divp: Some(PllDiv::Div5), // 80Mhz
            divq: Some(PllDiv::Div3), // 133.33Mhz
            divr: Some(PllDiv::Div2), // 200Mhz
        });
        config.rcc.pll3 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::Div25, // 2Mhz
            mul: PllMul::Mul96,     // 192Mhz
            divp: Some(PllDiv::Div1), // 192Mhz
            divq: Some(PllDiv::Div8), // 24Mhz
            divr: Some(PllDiv::Div4), // 48Mhz
        });
        config.rcc.voltage_scale = VoltageScale::Scale0;
        config.rcc.supply_config = SupplyConfig::DirectSMPS;
        config.rcc.sys = Sysclk::Pll1P; // 550Mhz
        config.rcc.d1c_pre = AHBPrescaler::Div1; // 550Mhz
        config.rcc.ahb_pre = AHBPrescaler::Div2; // 275Mhz
        config.rcc.apb1_pre = APBPrescaler::Div2; // 137.5Mhz
        config.rcc.apb2_pre = APBPrescaler::Div2; // 137.5Mhz
        config.rcc.apb3_pre = APBPrescaler::Div2; // 137.5Mhz
        config.rcc.apb4_pre = APBPrescaler::Div2; // 137.5Mhz
        embassy_stm32::init(config)
    }
}
