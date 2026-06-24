#![no_std]
#![no_main]
mod limine_requests;

use core::panic::PanicInfo;

use crate::limine_requests::{BASE_REVISION, FRAMEBUFFER_REQUEST};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if BASE_REVISION.is_supported() {}
    loop {}
}
