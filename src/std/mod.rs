#[prelude_import]
#[allow(unused)]
use prelude::rust_2021::*;

// FIXME: #94122 this extern crate definition only exist here to stop
// miniz_oxide docs leaking into std docs. Find better way to do it.
// Remove exclusion from tidy platform check when this removed.
// #[doc(masked)]
// #[allow(unused_extern_crates)]
// #[cfg(all(
//     not(all(windows, target_env = "msvc", not(target_vendor = "uwp"))),
//     feature = "miniz_oxide"
// ))]
//extern crate miniz_oxide;

// During testing, this crate is not actually the "real" std library, but rather
// it links to the real std library, which was compiled from this same source
// code. So any lang items std defines are conditionally excluded (or else they
// would generate duplicate lang item errors), and any globals it defines are
// _not_ the globals used by "real" std. So this import, defined only during
// testing gives test-std access to real-std lang items and globals. See #2912
//#[cfg(test)]
//extern crate std as realstd;

//The standard macros that are not built-in to the compiler.

//TODO:fix it: error[E0433]: failed to resolve: unresolved import
// #[macro_use]
// mod macros;

// The runtime entry point and a few unstable public functions used by the
// compiler
#[macro_use]
pub mod rt;

// The Rust prelude
pub mod prelude;
extern crate alloc as alloc_crate;
// Public module declarations and re-exports
pub use alloc_crate::borrow;
pub use alloc_crate::boxed;
pub use alloc_crate::fmt;
pub use alloc_crate::format;
pub use alloc_crate::rc;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::vec;
pub use core::any;
pub use core::array;
pub use core::async_iter;
pub use core::cell;
pub use core::char;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::future;
pub use core::hash;
pub use core::hint;
#[allow(deprecated, deprecated_in_future)]
pub use core::i128;
#[allow(deprecated, deprecated_in_future)]
pub use core::i16;
#[allow(deprecated, deprecated_in_future)]
pub use core::i32;
#[allow(deprecated, deprecated_in_future)]
pub use core::i64;
#[allow(deprecated, deprecated_in_future)]
pub use core::i8;
pub use core::intrinsics;
#[allow(deprecated, deprecated_in_future)]
pub use core::isize;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::option;
pub use core::pin;
pub use core::ptr;
pub use core::result;
#[allow(deprecated, deprecated_in_future)]
pub use core::u128;
#[allow(deprecated, deprecated_in_future)]
pub use core::u16;
#[allow(deprecated, deprecated_in_future)]
pub use core::u32;
#[allow(deprecated, deprecated_in_future)]
pub use core::u64;
#[allow(deprecated, deprecated_in_future)]
pub use core::u8;
#[allow(deprecated, deprecated_in_future)]
pub use core::usize;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod backtrace;
pub mod collections;
pub mod env;
pub mod error;
pub mod ffi;
pub mod fs;
pub mod io;
pub mod net;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod process;
pub mod sync;
pub mod time;

// Pull in `std_float` crate  into std. The contents of
// `std_float` are in a different repository: rust-lang/portable-simd.
// #[allow(missing_debug_implementations, dead_code, unsafe_op_in_unsafe_fn)]
// #[allow(rustdoc::bare_urls)]
// mod std_float;

// #[doc = include_str!("../../portable-simd/crates/core_simd/src/core_simd_docs.md")]
// pub mod simd {
//     #[doc(inline)]
//     pub use crate::std::std_float::StdFloat;
//     #[doc(inline)]
//     pub use core::simd::*;
// }

pub mod task {
    //! Types and Traits for working with asynchronous tasks.

    #[doc(inline)]
    pub use core::task::*;

    #[doc(inline)]
    pub use alloc::task::*;
}

// #[doc = include_str!("../../stdarch/crates/core_arch/src/core_arch_docs.md")]
// pub mod arch {
//         // The `no_inline`-attribute is required to make the documentation of all
//     // targets available.
//     // See https://github.com/rust-lang/rust/pull/57808#issuecomment-457390549 for
//     // more information.
//     #[doc(no_inline)] // Note (#82861): required for correct documentation
//     pub use core::arch::*;

//         pub use std_detect::is_aarch64_feature_detected;
//         pub use std_detect::is_x86_feature_detected;
//         pub use std_detect::{
//         is_arm_feature_detected, is_mips64_feature_detected, is_mips_feature_detected,
//         is_powerpc64_feature_detected, is_powerpc_feature_detected, is_riscv_feature_detected,
//     };
// }

// This was stabilized in the crate root so we have to keep it there.
//pub use std_detect::is_x86_feature_detected;

// Platform-abstraction modules
mod sys;
mod sys_common;

pub mod alloc;

// Private support modules
mod panicking;

pub use panicking::panic_hook_with_disk_dump;

// Re-export macros defined in core.
#[allow(deprecated, deprecated_in_future)]
pub use core::{
    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, r#try, todo,
    unimplemented, unreachable, write, writeln,
};

// Re-export built-in macros defined through core.
#[allow(deprecated)]
pub use core::{
    assert, assert_matches, cfg, column, compile_error, concat, concat_idents, const_format_args,
    env, file, format_args, format_args_nl, include, include_bytes, include_str, line, log_syntax,
    module_path, option_env, stringify, trace_macros,
};

pub use core::concat_bytes;

pub use core::primitive;

// Include a number of private modules that exist solely to provide
// the rustdoc documentation for primitive types. Using `include!`
// because rustdoc only looks for these modules at the crate level.
include!("primitive_docs.rs");

// Include a number of private modules that exist solely to provide
// the rustdoc documentation for the existing keywords. Using `include!`
// because rustdoc only looks for these modules at the crate level.
include!("keyword_docs.rs");

// This is required to avoid an unstable error when `restricted-std` is not
// enabled. The use of #![feature(restricted_std)] in rustc-std-workspace-std
// is unconditional, so the unstable feature needs to be defined somewhere.
mod __restricted_std_workaround {}

mod sealed {
    /// This trait being unreachable from outside the crate
    /// prevents outside implementations of our extension traits.
    /// This allows adding more trait methods in the future.
    pub trait Sealed {}
}
//pub use dlibc as libc;
