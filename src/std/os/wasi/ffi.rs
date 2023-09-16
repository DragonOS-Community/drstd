//! WASI-specific extensions to primitives in the [`std::ffi`] module
//!
//! [`std::ffi`]: crate::std::ffi

#[path = "../unix/ffi/os_str.rs"]
mod os_str;

pub use self::os_str::{OsStrExt, OsStringExt};
