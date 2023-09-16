//! Horizon OS raw type definitions

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
use dlibc;

pub type pthread_t = dlibc::pthread_t;

pub type blkcnt_t = dlibc::blkcnt_t;

pub type blksize_t = dlibc::blksize_t;
pub type dev_t = dlibc::dev_t;
pub type ino_t = dlibc::ino_t;
pub type mode_t = dlibc::mode_t;
pub type nlink_t = dlibc::nlink_t;
pub type off_t = dlibc::off_t;

pub type time_t = dlibc::time_t;

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
    pub st_mtime: time_t,
    pub st_ctime: time_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_spare4: [c_long; 2usize],
}
