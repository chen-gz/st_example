#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use defmt_rtt as _;
use u5_lib::*;
use u5_lib::gpio::{I2C1_SCL_PB6, I2C1_SDA_PB7};
use u5_lib::hal::I2c;

#[cortex_m_rt::entry]
fn main() -> ! {
    low_power::Executor::take().run(|spawner| {
        clock::init_clock(false, true, 4_000_000, true, clock::ClockFreqs::KernelFreq160Mhz);
        u5_lib::rtc::enable_rtc_read();
        defmt::info!("setup led finished!");
        let mut i2c = i2c::I2c::new(hal::I2cFrequency::Freq400khz, I2C1_SDA_PB7, I2C1_SCL_PB6).unwrap();
        let mut read_buf = [0;1];
        // i2c.read(0x36 , &mut read_buf).unwrap();
        // i2c.write(0x36 , &[0x01, 0x00]).unwrap();
        i2c.write_read(0x36, &[0x0C], &mut read_buf).unwrap();
        defmt::info!("send");
    });

}

