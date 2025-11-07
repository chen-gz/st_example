#![no_std]
#![no_main]

use core::time::Duration;
use defmt_rtt as _;
use u5_lib::gpio::{I2C1_SCL_PB6, I2C1_SDA_PB7, USART_RX_PA10, USART_TX_PA9};
use u5_lib::hal::I2c;
use u5_lib::hal::Usart;
use u5_lib::low_power::no_deep_sleep_request;
use u5_lib::rtc::rtc_delay;
use u5_lib::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    low_power::Executor::take().run(|spawner| {
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
        // low_power::no_deep_sleep_request();
        defmt::info!("init...");
        u5_lib::rtc::enable_rtc_read();
        spawner.spawn(serial_send()).unwrap();
    });
}

#[task]
async fn serial_send() {
    let mut usart = u5_lib::usart::Usart::new(115200, USART_TX_PA9, USART_RX_PA10).unwrap();
    loop {
        // let mut buf = [0u8; 64];
        // defmt::info!("sending...");
        // usart.write("hello world".as_ref()).unwrap();
        rtc_delay(Duration::from_secs(2)).await;
    }

    // loop {
    //     defmt::info!("waiting for rtc interrupt");
    //     rtc::rtc_delay(Duration::from_secs(3)).await;
    //     defmt::info!("rtc tick");
    //     // BLUE.toggle();
    // }
}
