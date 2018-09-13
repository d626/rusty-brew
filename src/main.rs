#![no_std]
#![no_main]

extern crate panic_abort;

extern crate cortex_m_rt;
use cortex_m_rt::entry;

extern crate cortex_m;

extern crate nrf52_hal;

fn busy_wait() {
    for _ in 0..100000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    let p = nrf52_hal::nrf52::Peripherals::take().unwrap();
    let p0 = p.P0;
    let _y;
    let x = 42;
    _y = x;

    p0.dirset.modify(|_r, w| w.pin17().set_bit());
    loop {
        p0.out.modify(|_r, w| w.pin17().set_bit());

        busy_wait();

        p0.out.modify(|_r, w| w.pin17().clear_bit());

        busy_wait();
    }
}
