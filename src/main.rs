#![no_main]
#![no_std]

use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m::asm::bkpt;

use nrf52840_hal as hal;

use hal::pac::Peripherals;
use hal::Temp;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, World!");
    
    let periph = Peripherals::take().unwrap();
    let mut temp_sensor = Temp::new(periph.TEMP);

    let temp: i32 = temp_sensor.measure().to_num();

    rprintln!("cpu temp is {:}ºC", temp);
    
    loop {
        bkpt();
    }
}
