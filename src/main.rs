#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, World!");
    exit();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// Example 2

// use cortex_m::asm;
// use cortex_m_rt::entry;
// use panic_probe as _;
// use rtt_target::{rprintln, rtt_init_print};

// #[entry]
// fn main() -> ! {
//     rtt_init_print!(); // You may prefer to initialize another way
//     rprintln!("Hello, world!");
//     loop { asm::bkpt() }
// }
