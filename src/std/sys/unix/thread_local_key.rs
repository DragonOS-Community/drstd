#![allow(dead_code)] // not used on all platforms

use crate::std::mem;
use dlibc;

pub type Key = dlibc::pthread_key_t;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let mut key = 0;
    assert_eq!(dlibc::pthread_key_create(&mut key, mem::transmute(dtor)), 0);
    key
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let r = dlibc::pthread_setspecific(key, value as *mut _);
    debug_assert_eq!(r, 0);
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    dlibc::pthread_getspecific(key) as *mut u8
}

#[inline]
pub unsafe fn destroy(key: Key) {
    let r = dlibc::pthread_key_delete(key);
    debug_assert_eq!(r, 0);
}
