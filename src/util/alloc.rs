use std::{alloc::Layout, ops::Deref};

struct Wrapper<T> {
    ptr: *const T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Wrapper<T> {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            *(ptr as *mut T) = value;
            ptr
        };
        Wrapper {
            ptr: ptr as *const T,
        }
    }

    fn get_value(&self) -> i32 {
        unsafe { *(self.ptr as *const i32) }
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // let v = self.get_value();
        unsafe { &*(self.ptr as *const Self::Target) }
    }
}

pub fn main() {
    let w = Wrapper::new(4);
    let w2 = Wrapper::new("hello world");
    println!("value1: {}, value2: {}", *w, *w2);
}
