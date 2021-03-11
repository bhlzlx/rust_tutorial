use std::alloc::*;

struct HeapPage<cap:u32> {
    let ptr: *mut u8,   // 内存指针
    let position: u32;  // 分配位置
}

impl HeapPage<cap:u32> {
    fn new()->HeapPage {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let memlayout = Layout::from_size_align_unchecked(cap, 16);
            ptr = alloc(memlayout);
        }
        let page: HeapPage = HeapPage {
            ptr: ptr,
            position: 0
        }
        return page;
    }
}