#![no_std]
#![no_main]

use defmt_rtt as _;
use u5_lib::*;
const GREEN: gpio::GpioPort = gpio::PB7;

// use u5_lib::low_power::Executor;
#[cortex_m_rt::entry]
fn main() -> ! {
    low_power::Executor::take().run(|spawner| {
        // clock::init_clock(false, true, 4_000_000, true, clock::ClockFreqs::KernelFreq160Mhz);
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
        GREEN.setup();
        u5_lib::rtc::enable_rtc_read();
        defmt::info!("setup led finished!");
        spawner.spawn(btn()).unwrap();
    });
}

#[task]
async fn btn() {
    let _last_time: (u8, u8, u8) = (0, 0, 0);
    defmt::info!("waiting for btn");
    loop {
        defmt::info!("button clicked");
        exti::EXTI13_PC13_PD.wait_for_raising().await;
        GREEN.toggle();
    }
}
