use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        println!("Kernel panic: {}", args);
    } else {
        println!("Kernel panic!");
    }
    // wait forever here
    unsafe {
        loop {
            asm!("wfe" :::: "volatile")
        }
    }
}
