//! sys/time implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/systime.h.html

use crate::unix::{c_str::CStr, platform};

pub use itimerval;
pub use timeval;

pub const ITIMER_REAL: ::c_int = 0;
pub const ITIMER_VIRTUAL: ::c_int = 1;
pub const ITIMER_PROF: ::c_int = 2;

#[repr(C)]
#[derive(Default)]
pub struct timezone {
    pub tz_minuteswest: ::c_int,
    pub tz_dsttime: ::c_int,
}

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [::c_long; 16usize],
}

// #[no_mangle]
// pub extern "C" fn getitimer(which: ::c_int, value: *mut itimerval) -> ::c_int {
//     platform::pal::getitimer(which, value)
// }

// #[no_mangle]
// pub extern "C" fn setitimer(
//     which: ::c_int,
//     value: *const itimerval,
//     ovalue: *mut itimerval,
// ) -> ::c_int {
//     platform::pal::setitimer(which, value, ovalue)
// }

#[no_mangle]
pub extern "C" fn gettimeofday(tp: *mut timeval, tzp: *mut timezone) -> ::c_int {
    #[cfg(target_os = "dragonos")]
    crate::unix::platform::dragonos::pal::relibc_adapter::pal::gettimeofday(
        tp,
        tzp as *mut ::c_void,
    )
}

#[no_mangle]
pub unsafe extern "C" fn utimes(path: *const ::c_char, times: *const timeval) -> ::c_int {
    let path = CStr::from_ptr(path);
    let times_spec = [
        ::timespec {
            tv_sec: (*times.offset(0)).tv_sec,
            tv_nsec: ((*times.offset(0)).tv_usec as ::c_long) * 1000,
        },
        ::timespec {
            tv_sec: (*times.offset(1)).tv_sec,
            tv_nsec: ((*times.offset(1)).tv_usec as ::c_long) * 1000,
        },
    ];
    platform::pal::utimens(path, times_spec.as_ptr())
}
