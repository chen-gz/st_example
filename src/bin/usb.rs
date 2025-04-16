#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use core::time::Duration;
use defmt_rtt as _;
use u5_lib::*;
use u5_lib::hal::Pin;

const GREEN: gpio::GpioPort = gpio::PB7;

use u5_lib::rtc;

use u5_lib::low_power::Executor;
use u5_lib::otg_fs::cdc_acm_ep2_write;
use u5_lib::otg_fs::control_pipe::setup_process;
use u5_lib::otg_fs::power::power_up_init;

#[cortex_m_rt::entry]
fn main() -> ! {
    Executor::take().run(|spawner| {
        clock::init_clock(false, true, 16_000_000, true, clock::ClockFreqs::KernelFreq160Mhz);
        low_power::no_deep_sleep_request();
        GREEN.setup();
        u5_lib::rtc::enable_rtc_read();
        defmt::info!("setup led finished!");
        defmt::trace!("setup led finished!");
        // spawner.spawn(btn()).unwrap();
        power_up_init();
        spawner.spawn(rtc()).unwrap();
        spawner.spawn(setup_process()).unwrap();
        spawner.spawn(usb_write()).unwrap();
    });
}

#[task]
async fn rtc() {
    loop {
        defmt::info!("waiting for rtc interrupt");
        rtc::rtc_delay(Duration::from_secs(3)).await;
        // cdc_acm_ep2_write("hello world from usb".as_bytes()).await;
        defmt::info!("rtc tick");
        GREEN.toggle();
    }
}

#[task]
async fn usb_write() {
    rtc::rtc_delay(Duration::from_secs(3)).await;
    loop {
        cdc_acm_ep2_write("hello world from usb".as_bytes()).await;
    }
}