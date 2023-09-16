#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

use crate::std::os::raw::c_int;

pub type dev_t = u32;
pub type mode_t = u32;

pub type pthread_t = c_int;

#[doc(inline)]
pub use self::arch::{blkcnt_t, blksize_t, ino_t, nlink_t, off_t, time_t};

mod arch {
    use crate::std::os::raw::c_long;

    pub type blkcnt_t = i64;
    pub type blksize_t = i32;
    pub type ino_t = u64;
    pub type nlink_t = u32;
    pub type off_t = i64;
    pub type time_t = c_long;
}
