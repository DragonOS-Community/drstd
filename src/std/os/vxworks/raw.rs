//! VxWorks-specific raw type definitions

use crate::std::os::raw::c_ulong;

pub type pthread_t = c_ulong;

pub use dlibc::{blkcnt_t, blksize_t, dev_t, ino_t, mode_t, nlink_t, off_t, time_t};
