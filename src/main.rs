#![no_main]
#![no_std]

use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
// use nrf52840_hal as _; // memory layout

// use nrf52840_hal::pac::Peripherals;
// use nrf52840_hal::Temp;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, World!");
    
    // let periph = Peripherals::take().unwrap();
    // let mut temp_sensor = Temp::new(periph.TEMP);

    // let temp: i32 = temp_sensor.measure().to_num();

    // defmt::info!("cpu temp is {:}ºC", temp);
    
    brk();
}

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     defmt::error!("panicked");
//     exit()
// }

pub fn brk() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
