use core::ffi::c_long;

use crate::platform::types::time_t;

#[repr(C)]
#[derive(Default)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}