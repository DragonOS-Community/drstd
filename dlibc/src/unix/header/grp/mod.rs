//! grp implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/grp.h.html

#[repr(C)]
pub struct group {
    pub gr_name: *mut ::c_char,
    pub gr_passwd: *mut ::c_char,
    pub gr_gid: ::gid_t,
    pub gr_mem: *mut *mut ::c_char,
}

// #[no_mangle]
pub extern "C" fn getgrgid(_gid: ::gid_t) -> *mut group {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn getgrnam(_name: *const ::c_char) -> *mut group {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn getgrgid_r(
    _gid: ::gid_t,
    _grp: *mut group,
    _buffer: *mut ::c_char,
    _bufsize: usize,
    _result: *mut *mut group,
) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn getgrnam_r(
    _name: *const ::c_char,
    _grp: *mut group,
    _buffer: *mut ::c_char,
    _bufsize: usize,
    _result: *mut *mut group,
) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn getgrent() -> *mut group {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn endgrent() {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn setgrent() {
    unimplemented!();
}

/*
#[no_mangle]
pub extern "C" fn func(args) -> ::c_int {
    unimplemented!();
}
*/
