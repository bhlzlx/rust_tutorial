pub struct ExcelString {
    ptr:*const u8
}

impl ExcelString {
    pub fn length(&self)->u16 {
        unsafe {
            return *(self.ptr as *const u16);
        }
    }
    pub fn text(&self)->*const char {
        return ((self.ptr as usize) + std::mem::size_of::<u16>()) as *const char;
    }
}

pub struct ExcelArray<T> {
    ptr:*const u8,
    marker: std::marker::PhantomData<*const T>,
}

impl<T> ExcelArray<T> {
    pub fn size(&self)->u16 {
        unsafe {
            return *(self.ptr as *const u16);
        }
    }

    pub fn get_elements(&self)->*const T {
        return (self.ptr as usize + std::mem::size_of::<u16>()) as *const T;
    }
}

