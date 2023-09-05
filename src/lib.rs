#![no_std]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(naked_functions)]
#![feature(start)]
#![feature(thread_local)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(lang_items)]

#[macro_use]
extern crate alloc;

#[macro_use]
pub mod macros;

#[macro_use]
extern crate memoffset;

//TODO: fix this: adjust to dragonos sc
#[cfg(target_os = "dragonos")]
#[macro_use]
extern crate dsc;

use crate::platform::allocator::{Allocator, NEWALLOCATOR};

pub mod header;
pub mod ld_so;
pub mod platform;
pub mod start;
pub mod fs;
pub mod sync;
pub mod c_str;
pub mod c_vec;
pub mod crt0;

mod io;

#[global_allocator]
static ALLOCATOR: Allocator = NEWALLOCATOR;

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

// #[cfg(not(target_os = "linux"))]
//#[no_mangle]
#[panic_handler]
fn panic_handle_func(info: &PanicInfo) -> ! {
    loop {}
}
