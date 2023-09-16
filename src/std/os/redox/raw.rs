//! Redox-specific raw type definitions

#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

use crate::std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

pub type dev_t = c_long;
pub type gid_t = c_int;
pub type mode_t = c_int;
pub type uid_t = c_int;

pub type pthread_t = *mut c_void;

pub type blkcnt_t = c_ulong;
pub type blksize_t = c_ulong;
pub type ino_t = c_ulong;
pub type nlink_t = c_ulong;
pub type off_t = c_long;
pub type time_t = c_long;

#[repr(C)]
#[derive(Clone)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_nlink: nlink_t,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atime: time_t,
    pub st_atime_nsec: c_long,
    pub st_mtime: time_t,
    pub st_mtime_nsec: c_long,
    pub st_ctime: time_t,
    pub st_ctime_nsec: c_long,
    pub _pad: [c_char; 24],
}
