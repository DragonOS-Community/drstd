use core::{alloc::GlobalAlloc, ffi::c_void, ptr::null_mut};

extern "C" {
    fn _dragonos_free(ptr: *mut c_void) -> *mut c_void;
    fn _dragonos_malloc(size: usize) -> *mut c_void;
    fn _dragonos_chunk_length(ptr: *mut c_void) -> usize;
}

pub struct Allocator;

pub const NEWALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = align_up(layout.size(), layout.align());

        return alloc(size);
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        free(ptr);
    }
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let size = align_up(new_size, layout.align());
        return realloc(ptr, size);
    }
}

pub unsafe fn alloc(size: usize) -> *mut u8 {
    return _dragonos_malloc(size) as *mut u8;
}

pub unsafe fn free(ptr: *mut u8) {
    _dragonos_free(ptr as *mut c_void);
}

fn align_up(addr: usize, align: usize) -> usize {
    return (addr + align - 1) & !(align - 1);
}

pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    if ptr.is_null() {
        return alloc(size);
    }
    if size == 0 {
        free(ptr);
        return null_mut();
    }

    let old_len = _dragonos_chunk_length(ptr as *mut c_void) - 16;

    // 暴力实现

    let new_ptr = alloc(size);
    if new_ptr.is_null() {
        return null_mut();
    }

    let copy_len = if old_len < size { old_len } else { size };
    core::ptr::copy_nonoverlapping(ptr, new_ptr, copy_len);

    free(ptr);

    return new_ptr;
}
