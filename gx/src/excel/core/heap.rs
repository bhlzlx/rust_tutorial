use std::alloc::*;

pub struct HeapPage {
    ptr: *mut u8,   // 内存指针
    position: usize,  // 分配位置
    capacity: usize,
}

impl HeapPage {

    pub fn new( capacity: usize)->HeapPage {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let memlayout = Layout::from_size_align_unchecked(capacity as usize, 16);
            ptr = alloc(memlayout);
            let page: HeapPage = HeapPage {
                ptr: ptr,
                position: 0,
                capacity: capacity,
            };
            return page;
        }
    }

    pub fn alloc(&mut self, size: usize)->*mut u8 {
        if (self.capacity - self.position) >= size {
            let address = self.ptr as usize + self.position;
            let rst = address as *mut u8;
            self.position += size;
            return rst;
        } else {
            return std::ptr::null_mut();
        }
    }
}

impl Drop for HeapPage {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.capacity as usize, 16);
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}
/*
pub struct Heap {
    page_capacity: usize,
    initial_page: HeapPage,
    current_page: NonNull<&mut HeapPage>,
}

impl Heap {
    fn new(page_capacity: usize)->Heap {
        // let heap_page = HeapPage::new(page_capacity);
        let heap: Heap;
        Heap {
            page_capacity: page_capacity,
            initial_page: HeapPage::new(page_capacity),
            current_page: NonNull::new_unchecked(&heap.initial_page as *mut HeapPage)
        };
        return heap;
    }
}*/