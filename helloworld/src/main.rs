mod gx {
    mod excel {
        mod heap {

            struct HeapPage {
                capacity: usize,
                position: usize,
                prev:*mut HeapPage,
                bytes: u8
            }

            impl HeapPage {

                fn new_ptr(capacity: usize)->*mut HeapPage {
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

                fn allocate( &mut self, size: usize)->*mut u8 {
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

            struct Heap {
                page: std::ptr::NonNull<HeapPage>,
                bytes_allcated: usize,
                bytes_heap_page: usize
            }

            impl Heap {
                
                fn new( page_size: usize)->Heap {
                    unsafe {
                        let page_ptr = HeapPage::new_ptr(page_size);
                        let heap:Heap = Heap {
                            page: std::ptr::NonNull::new_unchecked(page_ptr),
                            bytes_allcated: 0,
                            bytes_heap_page: 0,
                        };
                        return heap;
                    }
                }
                
                fn allocate( size: usize)-> *mut u8 {

                }
            }
        }
    }
}

fn main() {
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(1024, 8);
        let ptr = std::alloc::alloc(layout);
    }
}