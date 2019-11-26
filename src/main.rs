#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

mod panic_handler;

global_asm!(include_str!("../asm/start.S"));

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // start.S -> init bss -> jump to main
    // panic and loop from rust panic impl
    // panic -> asm wfe
    panic!()
}
