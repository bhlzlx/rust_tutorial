use std::alloc::*;

struct HeapPage {
    prev: *mut HeapPage,
    position: usize,
    capacity: usize,
}

impl HeapPage {

    fn address(&self)->*mut u8 {
        let page_ptr = self as *const HeapPage;
        let addr:*mut u8 = (page_ptr as usize + self.position) as *mut u8;
        return addr;
    }

    fn allocate(&mut self, size: usize)->*mut u8 {
        if self.capacity - self.position < size {
            return std::ptr::null_mut();
        } else {
            let rst = self.address();
            self.position += size;
            return rst;
        }
    }

}

pub struct Heap {
    page: *mut HeapPage,
    page_count: usize,
    page_size:usize,
    bytes_allocated: usize,
    bytes_total: usize,
}

impl Heap {

    fn create_page(size:usize, prev: *mut HeapPage)->*mut HeapPage {
        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size + std::mem::size_of::<HeapPage>(),16);
            let ptr = std::alloc::alloc(layout);
            let page = ptr as *mut HeapPage;
            let page_ref = &mut *page;
            page_ref.prev = prev;
            page_ref.position = 0;
            page_ref.capacity = size;
            return page;
        }
    }

    pub fn create_heap(page_size:usize)->Heap {
        let rst = Heap {
            page: Heap::create_page(page_size, std::ptr::null_mut()),
            page_count:1,
            page_size: page_size,
            bytes_allocated: 0,
            bytes_total:page_size
        };
        return rst;
    }

    pub fn allocate(&mut self, size:usize)->*mut u8{
        unsafe {
            let page = &mut *self.page;
            let mut rst = page.allocate(size);
            if std::ptr::null_mut() == rst {
                let new_page = Heap::create_page(self.page_size, self.page);
                self.page = new_page;
                rst = (&mut *self.page).allocate(size);
                return rst;
            } else {
                return rst;
            }
        }
    }
}