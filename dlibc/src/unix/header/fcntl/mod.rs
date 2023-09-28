//! fcntl implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/fcntl.h.html

pub use self::platform::*;
use crate::unix::platform;

#[no_mangle]
pub unsafe extern "C" fn creat(path: *const ::c_char, mode: ::mode_t) -> ::c_int {
    sys_open(path, O_WRONLY | O_CREAT | O_TRUNC, mode)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flock {
    pub l_type: ::c_short,
    pub l_whence: ::c_short,
    pub l_start: ::off_t,
    pub l_len: ::off_t,
    pub l_pid: ::pid_t,
}
#[no_mangle]
pub extern "C" fn sys_fcntl(fildes: ::c_int, cmd: ::c_int, arg: ::c_int) -> ::c_int {
    platform::pal::fcntl(fildes, cmd, arg)
}

#[no_mangle]
pub unsafe extern "C" fn sys_open(
    path: *const ::c_char,
    oflag: ::c_int,
    mode: ::mode_t,
) -> ::c_int {
    platform::pal::open(path, oflag, mode)
}

#[no_mangle]
pub unsafe extern "C" fn cbindgen_stupid_struct_user_for_fcntl(_a: flock) {}
