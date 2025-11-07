#![no_std]
#![no_main]
use cortex_m_rt::entry;
use defmt_rtt as _;
use gpio::GpioPort;
use u5_lib::clock;
use u5_lib::gpio;

const BLUE: GpioPort = gpio::PB7;
fn setup() {
    BLUE.setup();
}

#[entry]
fn main() -> ! {
    clock::init_clock(true, clock::ClockFreqs::KernelFreq4Mhz);
    setup();
    defmt::info!("setup led finished!");
    loop {
        BLUE.toggle();
        clock::delay_ms(500);
    }
}
