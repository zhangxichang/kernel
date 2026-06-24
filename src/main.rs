#![no_std]
#![no_main]
mod framebuffer;
mod limine_requests;

use core::panic::PanicInfo;

use crate::{
    framebuffer::draw_hello_world_to_framebuffer,
    limine_requests::{BASE_REVISION, FRAMEBUFFER_REQUEST},
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if BASE_REVISION.is_supported() {
        if let Some(response) = FRAMEBUFFER_REQUEST.response() {
            if let Some(framebuffer) = response.framebuffers().first() {
                draw_hello_world_to_framebuffer(framebuffer);
            }
        }
    }
    loop {}
}
