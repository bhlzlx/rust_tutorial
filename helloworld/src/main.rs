pub mod heap {

    pub struct HeapPage {
        capacity: usize,
        position: usize,
        prev:*mut HeapPage,
    }

    impl HeapPage {

        pub fn new_ptr(capacity: usize)->*mut HeapPage {
            unsafe {
                let full_size = capacity + std::mem::size_of::<HeapPage>();
                let layout = std::alloc::Layout::from_size_align_unchecked(full_size, std::mem::size_of::<usize>());
                let memptr = std::alloc::alloc(layout);
                let heap_page:*mut HeapPage = memptr as *mut HeapPage; 
                {
                    let heap_page_ref:&mut HeapPage = &mut (*heap_page);
                    heap_page_ref.capacity = capacity;
                    heap_page_ref.position = 0;
                    heap_page_ref.prev = std::ptr::null_mut();
                }
                return heap_page;
            }
        }

        pub fn buffer_address(&self)->usize {
            let ptr = self as *const HeapPage;
            let address = ptr as usize;
            return address + std::mem::size_of::<HeapPage>();
        }

        pub fn allocate( &mut self, size: usize)->*mut u8 {
            if self.capacity - self.position >= size {
                let address = self.buffer_address();
                let rst = (address + self.position) as *mut u8;
                self.position += size;
                return rst;
            } else {
                return std::ptr::null_mut();
            }
        }
    }

    pub struct Heap {
        current_page: *mut HeapPage,
        bytes_allcated: usize,
        bytes_heap_page: usize
    }

    impl Heap {
                
        pub fn new(page_size: usize)->Heap {
            let page_ptr = HeapPage::new_ptr(page_size);
            let heap:Heap = Heap {
                current_page: page_ptr,
                bytes_allcated: 0,
                bytes_heap_page: page_size,
            };
            return heap;
        }
                
        pub fn allocate(&mut self, size: usize)-> *mut u8 {
            unsafe {
                let current_page_ref = &mut *self.current_page;
                let mut rst = current_page_ref.allocate(size);
                if std::ptr::null_mut() == rst {
                    let new_page_ptr = HeapPage::new_ptr(self.bytes_heap_page);
                    let new_ref = &mut *new_page_ptr;
                    new_ref.prev = self.current_page;
                    self.current_page = new_page_ptr;
                    rst = new_ref.allocate(size);
                    return rst;
                } else {
                    return rst;
                }
            }
        }

        pub fn bytes_total(&self)->usize {
            return self.bytes_allcated;
        }
    }
}

fn main() {
    let page_size: usize = 1024;
    let mut heap: heap::Heap = heap::Heap::new(page_size);

    let mut rst: *mut u8 = std::ptr::null_mut();

    for _ in 0..5 {
        rst = heap.allocate(256);
        println!("{:?}", rst);
    }
}
