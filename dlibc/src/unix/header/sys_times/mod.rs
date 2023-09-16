//! sys/times.h implementation

use crate::unix::platform;

#[repr(C)]
pub struct tms {
    tms_utime: ::clock_t,
    tms_stime: ::clock_t,
    tms_cutime: ::clock_t,
    tms_cstime: ::clock_t,
}

#[no_mangle]
pub extern "C" fn times(out: *mut tms) -> ::clock_t {
    platform::pal::times(out)
}
