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
