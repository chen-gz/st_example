#![no_std]
#![no_main]

use defmt_rtt as _;
use dma::DCMI_DMA;
use embassy_executor::Spawner;
use gpio::{GpioPort, GPIO_MCO_PA8, PD12, PD13};
use hal::I2c;
use sdmmc::SD1;
use u5_lib::low_power::no_deep_sleep_request;
use u5_lib::*;

#[path = "../st_cam_v1.rs"]
mod st_cam_v1;
use st_cam_v1::*;

use stm32_metapac::{DBGMCU, RCC};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    _main(spawner).await;
}

async fn _main(spawner: Spawner) {
    // defmt::info!("begin system led finished!");
    unsafe { no_deep_sleep_request() };
    LED_GREEN.setup();
    LED_ORANGE.setup();
    LED_BLUE.setup();
    clock_init();
    let mut i2c = i2c_init();
    defmt::info!("i2c init finished!");
    u5_lib::drivers::ov5640::mod_new::setup_ov5640_camera(&mut i2c, &CAM_PWDN_PIN, &CAM_RESET_PIN);
    defmt::info!("camera init finished!");

    defmt::info!("begin init sd card");
    // let mut sd = u5_lib::sdmmc::SdInstance::new();
    // sd.init(clk, cmd, d0, d1, d2, d3, d4, d5, d6, d7);
    let mut read_data = &mut [0u8; 612_000];
    let dcmi = st_cam_v1::dcmi_init();
    let sd = st_cam_v1::sd_init();

    defmt::info!("begin capture");
    u5_lib::camera::capture(&CAM_PWDN_PIN, &mut i2c, &dcmi, read_data).await;
    defmt::info!("capture finished!");
    u5_lib::camera::save_picture(read_data, &sd).await;
    defmt::info!("save picture finished!");
    // sd.write_multiple_blocks(&read_data, 0, 1).await;
    // save to sd card
    spawner.spawn(btn()).unwrap();
}

#[embassy_executor::task]
async fn btn() {
    let _last_time: (u8, u8, u8) = (0, 0, 0);
    defmt::info!("waiting for btn");
    loop {
        exti::EXTI13_PC13.wait_for_raising().await;
        defmt::info!("button clicked");
        LED_GREEN.toggle();
    }
}

// clock::delay_ms(5000);
// let mut reg_val = [0u8; 3];
// reg_val[0] = (drivers::ov5640::ov5640_reg::OV5640_SYSTEM_CTROL0 >> 8) as u8;
// reg_val[1] = drivers::ov5640::ov5640_reg::OV5640_SYSTEM_CTROL0 as u8;
// reg_val[2] = (1 << 6) | 0x02;
// i2c.write(drivers::ov5640::ov5640_reg::OV5640_I2C_ADDR, &reg_val)
//     .unwrap();
// clock::delay_ms(200);

// use stm32_metapac::RCC;
// fn setup_cam_clk() {
//     let cam_clk: gpio::GpioPort = gpio::GPIO_MCO_PA8;
//     cam_clk.setup();
//     // set PA8 as mco output for HSI48 and divide by 2 (24Mhz)
//     RCC.cfgr1().modify(|w| {
//         w.set_mcosel(stm32_metapac::rcc::vals::Mcosel::HSI);
//         w.set_mcopre(stm32_metapac::rcc::vals::Mcopre::DIV1);
//     });
// }
//
// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//
//     _main(spawner).await;
// }

// async fn _main(spawner: Spawner) {
//     unsafe {no_deep_sleep_request()};
//     LED_GREEN.setup();
//     LED_ORANGE.setup();
//     LED_BLUE.setup();
//     LED_GREEN.set_high();;
//     LED_ORANGE.set_high();;
//     clock::init_clock(
//         true,
//         true,
//         26_000_000,
//         true,
//         clock::ClockFreqs::KernelFreq16Mhz,
//     );
//     setup_cam_clk();
//     LED_BLUE.set_high();;
//     defmt::info!("setup led finished!");
//     let mut i2c =
//         u5_lib::i2c::I2c::new(hal::I2cFrequency::Freq400khz, I2C2_SDA_PF0, I2C2_SCL_PF1).unwrap();
//
//         defmt::info!("i2c init finished!");
//     // let mut ret_data = [0u8; 1];
//
//     // u5_lib::drivers::ov5640::setup_camera(&mut i2c, &PD13, &PD12);
//     u5_lib::drivers::ov5640::mod_new::setup_ov5640_camera(&mut i2c, &PD13, &PD12);
//     defmt::info!("camera init finished!");
//     // i2c.write_read(
//     //     drivers::ov5640::ov5640_reg::OV5640_I2C_ADDR,
//     //     &drivers::ov5640::ov5640_reg::OV5640_CHIP_ID_HIGH_BYTE.to_be_bytes(),
//     //     &mut ret_data,
//     // ).unwrap();
//     // defmt::info!("ret_data: {:?}", ret_data);
//     spawner.spawn(btn()).unwrap();
// }
