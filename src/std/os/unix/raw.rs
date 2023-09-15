//! Unix-specific primitives available on all unix platforms.

#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

#[allow(non_camel_case_types)]
pub type uid_t = u32;

#[allow(non_camel_case_types)]
pub type gid_t = u32;

#[allow(non_camel_case_types)]
pub type pid_t = i32;

#[doc(inline)]
pub use super::platform::raw::pthread_t;
#[doc(inline)]
pub use super::platform::raw::{blkcnt_t, time_t};
#[doc(inline)]
pub use super::platform::raw::{blksize_t, dev_t, ino_t, mode_t, nlink_t, off_t};
