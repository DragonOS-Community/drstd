use crate::std::alloc::{GlobalAlloc, Layout, System};
use crate::std::ptr::null_mut;

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}

    #[inline]
    unsafe fn realloc(&self, _ptr: *mut u8, _layout: Layout, _new_size: usize) -> *mut u8 {
        null_mut()
    }
}
