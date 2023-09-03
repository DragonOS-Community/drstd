use crate::platform::types::{time_t, suseconds_t, c_int};

#[repr(C)]
#[derive(Default)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}
#[repr(C)]
#[derive(Default)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}