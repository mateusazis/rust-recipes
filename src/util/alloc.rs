use std::{alloc::Layout, fmt::Display, ops::Deref};

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
}

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr as *const Self::Target) }
    }
}

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reference = unsafe { &(*(self.ptr)) };
        reference.fmt(f)
    }
}

impl<T> Drop for Wrapper<T> {
    fn drop(&mut self) {
        println!("Dropping...");
        let layout = Layout::new::<T>();
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

pub fn main() {
    let w = Wrapper::new(4);
    let w2 = Wrapper::new("hello world");
    println!("value1: {}, value2: {}", w, w2);
}
