//! Emscripten-specific raw type definitions
//! This is basically exactly the same as the linux definitions,
//! except using the musl-specific stat64 structure in liblibc.

#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

use crate::std::os::raw::{c_long, c_short, c_uint, c_ulong};

pub type dev_t = u64;
pub type mode_t = u32;

pub type pthread_t = c_ulong;

pub type blkcnt_t = u64;
pub type blksize_t = u64;
pub type ino_t = u64;
pub type nlink_t = u64;
pub type off_t = u64;
pub type time_t = c_long;

#[repr(C)]
#[derive(Clone)]
pub struct stat {
        pub st_dev: u64,
        pub __pad1: c_short,
        pub __st_ino: u32,
        pub st_mode: u32,
        pub st_nlink: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub st_rdev: u64,
        pub __pad2: c_uint,
        pub st_size: i64,
        pub st_blksize: i32,
        pub st_blocks: i64,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_ino: u64,
}
