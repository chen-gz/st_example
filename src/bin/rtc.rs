#![no_std]
#![no_main]
use defmt_rtt as _;
use u5_lib::*;

const BLUE: gpio::GpioPort = gpio::PB7;
const ORANGE: gpio::GpioPort = gpio::PC7;

#[cortex_m_rt::entry]
fn main() -> ! {
    low_power::Executor::take().run(|spawner| {
        clock::init_clock(
            false,
            false,
            16_000_000,
            true,
            clock::ClockFreqs::KernelFreq160Mhz,
        );
        BLUE.setup();
        ORANGE.setup();
        u5_lib::rtc::enable_rtc_read();
        defmt::info!("setup led finished!");
        spawner.spawn(rtc()).unwrap();
        spawner.spawn(rtc2()).unwrap();
    });
}

#[task]
async fn rtc() {
    loop {
        defmt::info!("waiting for rtc interrupt");
        rtc::rtc_delay(core::time::Duration::from_secs(3)).await;
        defmt::info!("rtc tick");
        BLUE.toggle();
    }
}

#[task]
async fn rtc2() {
    loop {
        defmt::info!("waiting for rtc interrupt");
        rtc::rtc_delay(core::time::Duration::from_secs(5)).await;
        defmt::info!("rtc2 tick");
        ORANGE.toggle();
    }
}
