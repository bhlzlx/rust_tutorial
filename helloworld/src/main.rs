// use openal;
use gx;
use gx::excel::*;

fn main() {
    let page_size: usize = 1024;
    let mut heap: core::heap::Heap = core::heap::Heap::new(page_size);
    let mut rst: *mut u8 = std::ptr::null_mut();
    for _ in 0..5 {
        rst = heap.allocate(256);
        println!("{:?}", rst);
    }
}
