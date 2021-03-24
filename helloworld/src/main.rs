// use openal;
use gx;
use gx::excel::*;

fn main() {
    let msg = "hello,world!";
    let size = msg.len();
    let msg_substr = &msg[1..3];
    // let arr:[i8; 0] ;
    // let size =  std::mem::size_of::<[i8;1]>();
    println!("{} {}", size, msg_substr);
    let page_size: usize = 1024;
    let mut heap: core::heap::Heap = core::heap::Heap::create_heap(page_size);
    let mut rst: *mut u8 = std::ptr::null_mut();
    for _ in 0..5 {
        rst = heap.allocate(256);
        println!("{:?}", rst);
    }
}
