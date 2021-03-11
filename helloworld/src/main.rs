extern crate gx;

use gx::excel::hello_excel;

struct Point<T> {
    x: T,
    y: T
}

fn main(){
    gx::excel::hello_excel();
    gx::excel::core::hello_core();
}