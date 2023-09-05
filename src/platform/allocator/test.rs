#[cfg(test)]
mod tests {
    use crate::platform::allocator::{Allocator, NEWALLOCATOR};
    use core::alloc::{GlobalAlloc, Layout};

    #[test]
    fn test_allocation_and_deallocation() {
        let allocator = NEWALLOCATOR;

        let size = 1024; // 分配 1024 字节的内存

        unsafe {
            // 使用 Allocator 分配内存
            let layout = Layout::from_size_align(size, 1).unwrap();
            let ptr = allocator.alloc(layout);

            assert!(!ptr.is_null()); // 确保分配成功

            // 使用 Allocator 释放内存
            allocator.dealloc(ptr, layout);
        }
    }

    #[test]
    fn test_alignment() {
        let allocator = NEWALLOCATOR;

        let size = 1024;
        let align = 16;

        unsafe {
            let layout = Layout::from_size_align(size, align).unwrap();
            let ptr = allocator.alloc(layout);

            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % align, 0);

            allocator.dealloc(ptr, layout);
        }
    }

    #[test]
    fn test_multiple_allocations() {
        let allocator = NEWALLOCATOR;


        let sizes = [100, 200, 300, 400, 500];

        unsafe {
            for &size in &sizes {
                let layout = Layout::from_size_align(size, 1).unwrap();
                let ptr = allocator.alloc(layout);

                assert!(!ptr.is_null());

                allocator.dealloc(ptr, layout);
            }
        }
    }

    #[test]
    fn test_large_allocation() {
        let allocator = NEWALLOCATOR;

        let size = 1024 * 1024; // 1MB

        unsafe {
            let layout = Layout::from_size_align(size, 1).unwrap();
            let ptr = allocator.alloc(layout);

            assert!(!ptr.is_null());

            allocator.dealloc(ptr, layout);
        }
    }
}
