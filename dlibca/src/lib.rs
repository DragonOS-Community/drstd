//! libc - Raw FFI bindings to platforms' system libraries
//!
//! [Documentation for other platforms][pd].
//!
//! [pd]: https://rust-lang.github.io/libc/#platform-specific-documentation
//! 

#![feature(core_intrinsics)]
#![feature(linkage)]
#![feature(thread_local)]
#![feature(start)]
#![feature(naked_functions)]
#![feature(slice_internals)]
#![feature(c_variadic)]
#![feature(stmt_expr_attributes)]
#![feature(lang_items)]


#![crate_name = "dlibc"]
#![crate_type = "staticlib"]
#![allow(
    renamed_and_removed_lints, // Keep this order.
    unknown_lints, // Keep this order.
    bad_style,
    overflowing_literals,
    improper_ctypes,
    // This lint is renamed but we run CI for old stable rustc so should be here.
    redundant_semicolon,
    redundant_semicolons,
    unused_macros,
    unused_macro_rules,
)]
#![cfg_attr(libc_deny_warnings, deny(warnings))]
// Attributes needed when building as part of the standard library
#![cfg_attr(feature = "rustc-dep-of-std", feature(link_cfg, no_core))]
#![cfg_attr(libc_thread_local, feature(thread_local))]
// Enable extra lints:
#![cfg_attr(feature = "extra_traits", deny(missing_debug_implementations))]
//#![deny(missing_copy_implementations, safe_packed_borrows)]
#![cfg_attr(not(feature = "rustc-dep-of-std"), no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]
#![cfg_attr(libc_const_extern_fn_unstable, feature(const_extern_fn))]

#[macro_use]
extern crate alloc;
extern crate core_io;
#[macro_use]
extern crate lazy_static;
extern crate memchr;
extern crate goblin;
extern crate rand;
extern crate posix_regex;
extern crate cbitset;
extern crate num_traits;


#[macro_use]
mod macros;

cfg_if! {
    if #[cfg(feature = "rustc-dep-of-std")] {
        extern crate rustc_std_workspace_core as core;
        #[allow(unused_imports)]
        use core::iter;
        #[allow(unused_imports)]
        use core::ops;
        #[allow(unused_imports)]
        use core::option;
    }
}
cfg_if! {
    if #[cfg(libc_priv_mod_use)] {
        #[cfg(libc_core_cvoid)]
        #[allow(unused_imports)]
        use core::ffi;
        #[allow(unused_imports)]
        use core::fmt;
        #[allow(unused_imports)]
        use core::hash;
        #[allow(unused_imports)]
        use core::num;
        #[allow(unused_imports)]
        use core::mem;
        #[doc(hidden)]
        #[allow(unused_imports)]
        use core::clone::Clone;
        #[doc(hidden)]
        #[allow(unused_imports)]
        use core::marker::{Copy, Send, Sync};
        #[doc(hidden)]
        #[allow(unused_imports)]
        use core::option::Option;
    } else {
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::fmt;
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::hash;
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::num;
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::mem;
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::clone::Clone;
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::marker::{Copy, Send, Sync};
        #[doc(hidden)]
        #[allow(unused_imports)]
        pub use core::option::Option;
    }
}

#[cfg(target_os = "dragonos")]
pub mod unix;
#[cfg(target_os = "dragonos")]
pub use unix::*;
#[cfg(target_os = "dragonos")]
#[macro_use]
extern crate dsc;
#[cfg(target_os = "dragonos")]
#[global_allocator]
static ALLOCATOR: crate::unix::platform::allocator::Allocator = crate::unix::platform::allocator::ALLOCATOR;
#[cfg(target_os = "dragonos")]
pub use crate::unix::macros::*;

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(target_os = "dragonos")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // 在这里执行自定义的处理逻辑
    println!("Panic occurred: {:?}", info);

    // 可以选择进行一些清理或其他操作

    // 结束程序，例如通过调用 `std::process::exit`
    crate::unix::platform::pal::exit(0);
}                        