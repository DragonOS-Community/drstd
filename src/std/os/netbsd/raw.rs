//! NetBSD-specific raw type definitions

#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

use crate::std::os::raw::c_long;
use crate::std::os::unix::raw::{gid_t, uid_t};

pub type blkcnt_t = u64;
pub type blksize_t = u64;
pub type dev_t = u64;
pub type fflags_t = u32;
pub type ino_t = u64;
pub type mode_t = u32;
pub type nlink_t = u64;
pub type off_t = u64;
pub type time_t = i64;

pub type pthread_t = usize;

#[repr(C)]
#[derive(Clone)]
pub struct stat {
        pub st_dev: u64,
        pub st_mode: u32,
        pub st_ino: u64,
        pub st_nlink: u32,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: u64,
        pub st_atime: i64,
        pub st_atime_nsec: c_long,
        pub st_mtime: i64,
        pub st_mtime_nsec: c_long,
        pub st_ctime: i64,
        pub st_ctime_nsec: c_long,
        pub st_birthtime: i64,
        pub st_birthtime_nsec: c_long,
        pub st_size: i64,
        pub st_blocks: i64,
        pub st_blksize: i32,
        pub st_flags: u32,
        pub st_gen: u32,
    st_spare: [u32; 2],
}
