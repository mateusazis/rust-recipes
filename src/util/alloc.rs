use std::{alloc::Layout, ops::Deref};

struct Wrapper {
    ptr: *const u8,
}

impl Wrapper {
    fn new(value: i32) -> Wrapper {
        let layout = Layout::new::<i32>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            *(ptr as *mut i32) = value;
            ptr
        };
        Wrapper { ptr }
    }

    fn get_value(&self) -> i32 {
        unsafe { *(self.ptr as *const i32) }
    }
}

impl Deref for Wrapper {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        let v = self.get_value();
        v
    }
}

pub fn main() {
    let w = Wrapper::new(4);
    println!("value: {}", w.get_value());
}
