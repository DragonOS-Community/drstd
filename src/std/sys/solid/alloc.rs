use crate::std::{
    alloc::{GlobalAlloc, Layout, System},
    sys::common::alloc::{realloc_fallback, MIN_ALIGN},
};
use dlibc;

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            unsafe { dlibc::malloc(layout.size()) as *mut u8 }
        } else {
            unsafe { dlibc::memalign(layout.align(), layout.size()) as *mut u8 }
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { dlibc::free(ptr as *mut dlibc::c_void) }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
                dlibc::realloc(ptr as *mut dlibc::c_void, new_size) as *mut u8
            } else {
                realloc_fallback(self, ptr, layout, new_size)
            }
        }
    }
}
