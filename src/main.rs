#![no_std]
#![no_main]

use cortex_m::{asm, peripheral::SCB};
use cortex_m_rt::{entry, exception};
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("main");
    SCB::set_pendsv(); // trigger PendSV exception
    rprintln!("after PendSV");

    exit()
}

#[exception]
fn PendSV() {
    rprintln!("PendSV");
    panic!()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    exit()
}

fn exit() -> ! {
    loop {
        asm::bkpt() // halt = exit probe-run
    }
}
