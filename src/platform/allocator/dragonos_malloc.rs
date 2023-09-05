use core::{alloc::GlobalAlloc, ptr::null_mut, sync::atomic::{AtomicUsize, Ordering}};

use super::types::c_void;

extern "C" {
    fn _dragonos_free(ptr: *mut c_void) -> *mut c_void;
    fn _dragonos_malloc(size: usize) -> *mut c_void;
    fn _dragonos_chunk_length(ptr: *mut c_void) -> usize;
}

pub struct Allocator{
    mstate: AtomicUsize,
}

impl Allocator {
    pub fn set_book_keeper(&self, mstate: usize) {
        self.mstate.store(mstate, Ordering::Relaxed);
    }
    pub fn get_book_keeper(&self) -> usize {
        self.mstate.load(Ordering::Relaxed)
    }
}

pub const NEWALLOCATOR: Allocator = Allocator{
    mstate: AtomicUsize::new(0),
};

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = align_up(layout.size(), layout.align());

        return alloc(size) as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        free(ptr as *mut c_void);
    }
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let size = align_up(new_size, layout.align());
        return realloc(ptr as *mut c_void, size) as *mut u8;
    }
}

pub unsafe fn alloc(size: usize) -> *mut c_void {
    return _dragonos_malloc(size);
}

pub unsafe fn free(ptr: *mut c_void) {
    _dragonos_free(ptr);
}

fn align_up(addr: usize, align: usize) -> usize {
    return (addr + align - 1) & !(align - 1);
}

pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
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

pub unsafe fn alloc_align(mut size: usize, alignment: usize) -> *mut c_void {
    // println!("alloc align size: {}, alignment: {}", size, alignment);
    size = align_up(size, alignment);

    // TODO: 实现对齐分配
    _dragonos_malloc(size)
    //mspace_memalign(ALLOCATOR.get_book_keeper(), alignment, size)
}

pub fn new_mspace() -> usize {
    // dbg!("new_mspace");
    1
}
