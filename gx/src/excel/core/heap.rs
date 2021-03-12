use std::alloc::*;

pub struct HeapPage<'a> {
    ptr: *mut u8,   // 内存指针
    position: u32,  // 分配位置
    capacity: u32,
    full_slice:&'a mut [u8],
}

impl HeapPage<'_> {
    pub fn new(capacity: u32)->HeapPage<'static> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let memlayout = Layout::from_size_align_unchecked(capacity as usize, 16);
            ptr = alloc(memlayout);
        
            let page: HeapPage = HeapPage {
                ptr: ptr,
                position: 0,
                capacity: capacity,
                full_slice: std::slice::from_raw_parts_mut(ptr, capacity as usize)
            };
            return page;
        }
    }

    pub fn alloc(&mut self, size: u32)->Option<&mut[u8]> {
        if (self.capacity - self.position) >= size {
            let begin = self.position as usize;
            let end = begin + size as usize;
            let rst = &mut self.full_slice[begin..end];
            self.position += size;
            return Some(rst);
        } else {
            return None;
        }
    }
}

impl Drop for HeapPage<'_> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.capacity as usize, 16);
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}

pub struct Heap {
    page_capacity: u32,
    initial_page: HeapPage,
    current_page: NonNull<&mut HeapPage>,
}

impl Heap {
    fn new(page_capacity: u32)->Heap {
        // let heap_page = HeapPage::new(page_capacity);
        let heap: Heap;
        Heap {
            page_capacity: page_capacity,
            initial_page: HeapPage::new(page_capacity),
            current_page: NonNull::new_unchecked(&heap.initial_page as *mut HeapPage)
        };
        return heap;
    }
}