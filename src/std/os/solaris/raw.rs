//! Solaris-specific raw type definitions

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

pub type pthread_t = u32;

#[repr(C)]
#[derive(Clone)]
pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_atime: time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: time_t,
        pub st_ctime_nsec: c_long,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub __unused: [u8; 16],
}
