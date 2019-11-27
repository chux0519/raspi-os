#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod panic_handler;
mod print;

global_asm!(include_str!("../asm/start.S"));

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // start.S -> init bss -> jump to main
    println!("hello from rust");
    // panic and loop from rust panic impl
    // panic -> asm wfe
    panic!("panic here")
}
