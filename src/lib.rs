#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use crate::platform::allocator::{Allocator, NEWALLOCATOR};

mod platform;
mod header;

#[global_allocator]
static ALLOCATOR: Allocator = NEWALLOCATOR;

#[panic_handler]
#[no_mangle]
#[cfg(not(target_os = "linux"))]
fn panic_handle_func(info: &PanicInfo) -> ! {
    loop {}
}
