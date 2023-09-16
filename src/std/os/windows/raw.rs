//! Windows-specific primitives.

use crate::std::os::raw::c_void;

pub type HANDLE = *mut c_void;
#[cfg(target_pointer_width = "32")]
#[doc(cfg(all()))]
pub type SOCKET = u32;
#[cfg(target_pointer_width = "64")]
#[doc(cfg(all()))]
pub type SOCKET = u64;
