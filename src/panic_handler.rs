use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // wait forever here
    unsafe {
        loop {
            asm!("wfe" :::: "volatile")
        }
    }
}
