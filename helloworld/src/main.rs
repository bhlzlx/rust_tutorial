use std::vec::*;
use std::boxed::*;

struct SparseColumn {
    lineCount: i32,
    ptr: *mut i8,
}

fn main() {
    let mut buf: Vec<i8> = Vec::new();
    buf.resize(1024, 0);
    let rawPtr = buf.as_ptr();

    unsafe {
        let mut col :*mut SparseColumn = rawPtr as * mut SparseColumn; 
        (*col).lineCount = 0;
    }
    
}
