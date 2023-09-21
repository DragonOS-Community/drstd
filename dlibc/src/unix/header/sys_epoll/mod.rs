//! sys/epoll.h implementation for Redox, following http://man7.org/linux/man-pages/man7/epoll.7.html

use core::ptr;



use crate::unix::platform;

pub use self::platform::*;

#[no_mangle]
pub unsafe extern "C" fn epoll_create(_size: ::c_int) -> ::c_int {
    ::epoll_create1(0)
}

#[no_mangle]
pub extern "C" fn epoll_wait(
    epfd: ::c_int,
    events: *mut epoll_event,
    maxevents: ::c_int,
    timeout: ::c_int,
) -> ::c_int {
    unsafe{epoll_pwait(epfd, events, maxevents, timeout, ptr::null())}
}
