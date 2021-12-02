#![no_std]
#![forbid(unsafe_code)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub extern "C" fn solution() -> [u8; 128] {
    let mut s = [0u8; 128];
    s[0] = b'h';
    s[1] = b'i';
    s[2] = b'!';
    s
}
