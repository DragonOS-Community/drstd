

#[repr(C)]
#[derive(Copy)]
pub union sem_t {
    pub size: [::c_char; 32usize],
    pub align: ::c_long,
    _bindgen_union_align: [u64; 4usize],
}
impl Clone for sem_t {
    fn clone(&self) -> Self {
        *self
    }
}
// #[no_mangle]
pub extern "C" fn sem_init(_sem: *mut sem_t, _pshared: ::c_int, _value: ::c_uint) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn sem_destroy(_sem: *mut sem_t) -> ::c_int {
    unimplemented!();
}

/*
 *#[no_mangle]
 *pub extern "C" fn sem_open(name: *const ::c_char,
 *                    oflag: ::c_int, ...) -> *mut sem_t {
 *    unimplemented!();
 *}
 */

// #[no_mangle]
pub extern "C" fn sem_close(_sem: *mut sem_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn sem_unlink(_name: *const ::c_char) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn sem_wait(_sem: *mut sem_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn sem_trywait(_sem: *mut sem_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn sem_post(_sem: *mut sem_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn sem_getvalue(_sem: *mut sem_t, _sval: *mut ::c_int) -> ::c_int {
    unimplemented!();
}
