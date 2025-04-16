use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // println!("cargo:rustc-link-arg-bins=-Tmemory.x"); // feagure in cortex-m-rt
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    let runner = if cfg!(feature = "stm32u5a5zj") {
        "probe-rs run --chip STM32U5A5ZJTx"
    } else if cfg!(feature = "stm32u575zi") {
        "probe-rs run --chip STM32U575ZITxQ"
    } else if cfg!(feature = "stm32u575ci") {
        "probe-rs run --chip STM32U575CIUxQ"
    } else {
        panic!("No chip selected");
    };
    // create folder .cargo and file config.toml
    if !std::path::Path::new(".cargo").exists() {
        std::fs::create_dir(".cargo").expect("Failed to create .cargo directory");
    }

    let config_path = Path::new(".cargo").join("config.toml");
    let mut config_file = File::create(&config_path).unwrap();
    writeln!(config_file, "[target.thumbv8m.main-none-eabihf]").unwrap();
    writeln!(config_file, "runner = \"{}\"", runner).unwrap();
    writeln!(config_file, "[build]").unwrap();
    writeln!(config_file, "target = \"thumbv8m.main-none-eabihf\"").unwrap();
    writeln!(config_file, "[env]").unwrap();
    writeln!(config_file, "DEFMT_LOG = \"trace\"").unwrap();
    // writeln!(config_file, "DEFMT_TIMESTAMP = \"1\"").expect("Failed to write to config file");
    env::set_var("CARGO_RUNNER", runner);

    // create memory.x file
    let memory_x_path = Path::new("memory.x");
    let mut memory_x_file = File::create(&memory_x_path).unwrap();
    if cfg!(feature = "stm32u5a5zj") {
        write!(memory_x_file, "MEMORY\n{{\n  FLASH : ORIGIN = 0x08000000, LENGTH = 4096K\n  RAM   : ORIGIN = 0x20000000, LENGTH =  2496K\n  OTP   : ORIGIN = 0x0bfa0000, LENGTH =  512\n}}\n").unwrap();
    } else {
        // stm32u575
        write!(memory_x_file, "MEMORY\n{{\n  FLASH : ORIGIN = 0x08000000, LENGTH = 4096K\n  RAM   : ORIGIN = 0x20000000, LENGTH =  768K\n  OTP   : ORIGIN = 0x0bfa0000, LENGTH =  512\n}}\n").unwrap();
    }
}
