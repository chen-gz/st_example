use u5_lib::{clock, dcmi, dma::DCMI_DMA, gpio, sdmmc::SD1};
pub const HSE_FREQ: u32 = 26_000_000; // 26Mhz
pub const LSE_FREQ: u32 = 32_768_000; // 32.768Khz
pub const DEBUG_ENABLE: bool = true;
pub const HSE_AVAILABLE: bool = true;
pub const LSE_AVAILABLE: bool = true;
pub const SYSTEM_DEFAULT_CLOCK: clock::ClockFreqs = clock::ClockFreqs::KernelFreq16Mhz;
pub const LED_GREEN: gpio::GpioPort = gpio::PD14;
pub const LED_ORANGE: gpio::GpioPort = gpio::PD15;
pub const LED_BLUE: gpio::GpioPort = gpio::PD10;
pub const I2C_SCL_PIN: gpio::GpioPort = gpio::I2C2_SCL_PF1;
pub const I2C_SDA_PIN: gpio::GpioPort = gpio::I2C2_SDA_PF0;
pub const CAM_PWDN_PIN: gpio::GpioPort = gpio::PD13;
pub const CAM_RESET_PIN: gpio::GpioPort = gpio::PD12;
pub const CAM_DCMI_D0: gpio::GpioPort = gpio::DCMI_D0_PA9;
pub const CAM_DCMI_D1: gpio::GpioPort = gpio::DCMI_D1_PA10;
pub const CAM_DCMI_D2: gpio::GpioPort = gpio::DCMI_D2_PE0;
pub const CAM_DCMI_D3: gpio::GpioPort = gpio::DCMI_D3_PE1;
pub const CAM_DCMI_D4: gpio::GpioPort = gpio::DCMI_D4_PE4;
pub const CAM_DCMI_D5: gpio::GpioPort = gpio::DCMI_D5_PB6;
pub const CAM_DCMI_D6: gpio::GpioPort = gpio::DCMI_D6_PE5;
pub const CAM_DCMI_D7: gpio::GpioPort = gpio::DCMI_D7_PE6;
pub const CAM_DCMI_HSYNC: gpio::GpioPort = gpio::DCMI_HSYNC_PA4;
pub const CAM_DCMI_VSYNC: gpio::GpioPort = gpio::DCMI_VSYNC_PB7;
pub const CAM_DCMI_PIXCLK: gpio::GpioPort = gpio::DCMI_PIXCLK_PD9;
pub const CAM_XCLK_PIN: gpio::GpioPort = gpio::GPIO_MCO_PA8;

pub const SD_CLK: gpio::GpioPort = gpio::SDMMC2_CK_PD6;
pub const SD_CMD: gpio::GpioPort = gpio::SDMMC2_CMD_PD7;
pub const SD_D0: gpio::GpioPort = gpio::SDMMC2_D0_PB14;
pub const SD_D1: gpio::GpioPort = gpio::SDMMC2_D1_PB15;
pub const SD_D2: gpio::GpioPort = gpio::SDMMC2_D2_PB3;
pub const SD_D3: gpio::GpioPort = gpio::SDMMC2_D3_PB4;
pub const SD_D4: gpio::GpioPort = gpio::SDMMC2_D4_PB8;
pub const SD_D5: gpio::GpioPort = gpio::SDMMC2_D5_PB9;
pub const SD_D6: gpio::GpioPort = gpio::SDMMC2_D6_PC6;
pub const SD_D7: gpio::GpioPort = gpio::SDMMC2_D7_PC7;

pub fn dcmi_init() -> dcmi::DcmiPort {
    let mut dcmi = dcmi::DCMI;
    DCMI_DMA.init();
    dcmi.init(
        CAM_DCMI_D0,
        CAM_DCMI_D1,
        CAM_DCMI_D2,
        CAM_DCMI_D3,
        CAM_DCMI_D4,
        CAM_DCMI_D5,
        CAM_DCMI_D6,
        CAM_DCMI_D7,
        CAM_DCMI_HSYNC,
        CAM_DCMI_VSYNC,
        CAM_DCMI_PIXCLK,
    );
    dcmi
}
pub fn sd_init() -> u5_lib::sdmmc::SdInstance {
    let mut sd = u5_lib::sdmmc::SdInstance::new(stm32_metapac::SDMMC2);
    sd.init(
        SD_CLK, SD_CMD, SD_D0, SD_D1, SD_D2, SD_D3, SD_D4, SD_D5, SD_D6, SD_D7,
    );
    sd
}
pub fn clock_init() {
    clock::init_clock(
        HSE_AVAILABLE,
        LSE_AVAILABLE,
        HSE_FREQ,
        DEBUG_ENABLE,
        SYSTEM_DEFAULT_CLOCK,
    );
    clock::set_mco(CAM_XCLK_PIN, clock::Mcosel::HSI48, clock::Mcopre::DIV2); // set mco (camera clock) to 24Mhz
}

use u5_lib::hal;
pub fn i2c_init() -> u5_lib::i2c::I2c {
    let mut i2c = u5_lib::hal::I2c::new(hal::I2cFrequency::Freq400khz, I2C_SDA_PIN, I2C_SCL_PIN).unwrap();
    i2c
}