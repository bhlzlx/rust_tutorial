pub mod gx {
    pub mod excel {
        pub mod heap {

            pub struct HeapPage {
                capacity: usize,
                position: usize,
                prev:*mut HeapPage,
                bytes: u8
            }

            impl HeapPage {

                pub fn new_ptr(capacity: usize)->*mut HeapPage {
                    unsafe {
                        let full_size = capacity + std::mem::size_of::<usize>()*3;
                        let layout = std::alloc::Layout::from_size_align_unchecked(full_size, std::mem::size_of::<usize>());
                        let memptr = std::alloc::alloc(layout);
                        let heapPage:*mut HeapPage = memptr as *mut HeapPage; 
                        {
                            let heapPageRef:&mut HeapPage = &mut (*heapPage);
                            heapPageRef.capacity = capacity;
                            heapPageRef.position = 0;
                            heapPageRef.prev = std::ptr::null_mut();
                        }
                        return heapPage;
                    }
                }

                pub fn allocate( &mut self, size: usize)->*mut u8 {
                    if self.capacity - self.position >= size {
                        let address = &mut self.bytes as *mut u8;
                        let rst = (address as usize + self.position) as *mut u8;
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
                    unsafe {
                        let page_ptr = HeapPage::new_ptr(page_size);
                        let heap:Heap = Heap {
                            current_page: page_ptr,
                            bytes_allcated: 0,
                            bytes_heap_page: page_size,
                        };
                        return heap;
                    }
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
            }
        }
    }
}

use gx::excel::*;

fn main() {
    unsafe {
        let page_size: usize = 1024;
        let mut heap: heap::Heap = heap::Heap::new(page_size);

        let mut rst: *mut u8 = std::ptr::null_mut();

        for number in (0..5) {
            rst = heap.allocate(256);
            println!("{:?}", rst);
        }
    }
}