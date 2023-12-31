use crate::std::alloc::{GlobalAlloc, Layout, System};
use crate::std::ptr;
use crate::std::sys::hermit::abi;

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        abi::malloc(layout.size(), layout.align())
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let addr = abi::malloc(layout.size(), layout.align());

        if !addr.is_null() {
            ptr::write_bytes(addr, 0x00, layout.size());
        }

        addr
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        abi::free(ptr, layout.size(), layout.align())
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        abi::realloc(ptr, layout.size(), layout.align(), new_size)
    }
}
