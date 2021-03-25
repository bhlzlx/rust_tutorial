struct MemoryChunk {
    prev:*const MemoryChunk,
    capacity: usize,
    position: usize,
    buffer:[u8;0]
}

impl MemoryChunk {
    fn new(capacity: usize, heap:*mut super::heap::Heap)->*mut MemoryChunk {
        unsafe {
            let heap_ref = &mut *heap;
            let size_need = std::mem::size_of::<MemoryChunk>() + capacity;
            let mem_ptr:*mut u8 = heap_ref.allocate(size_need);
            let rst:*mut MemoryChunk = mem_ptr as *mut MemoryChunk;
            let rst_ref = &mut *rst;
            rst_ref.position = 0;
            rst_ref.capacity = capacity;
            return rst;
        }
    }
    fn write_value<T>(&mut self, value: T )->bool {
        unsafe {
            let left = self.capacity - self.position;
            if left < std::mem::size_of::<T>() {
                return false;
            } else {
                let ptr = &self.buffer as *const u8 as *mut T;
                *ptr = value;
                self.position += std::mem::size_of::<T>();
                return true;
            }
        }
    }
}

struct ColumnStorage {
    chunk: *const MemoryChunk,
}

impl ColumnStorage {
    pub fn write<T>(value: T) {

    }
}

pub struct SparseStorage {

}