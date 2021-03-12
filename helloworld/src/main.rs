extern crate gx;

// use gx::excel::hello_excel;

use gx::excel::*;

struct Point<T> {
    x: T,
    y: T
}

fn main(){
    // gx::excel::hello_excel();
    // gx::excel::core::hello_core();
    let page: core::heap::HeapPage = core::heap::HeapPage::new(1024*64);
}