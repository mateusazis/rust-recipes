use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::vec::Vec;

// #[global_allocator]
// static mut GLOBAL: MyAllocator = MyAllocator::new();

struct MyAllocator<const T: usize> {
    data: [u8; T],
    free_from_here: [bool; T],
}

impl<const T: usize> MyAllocator<T> {
    const fn new() -> MyAllocator<T> {
        let data = [0u8; T];
        let free_from_here = [true; T];
        MyAllocator {
            data,
            free_from_here,
        }
    }

    fn init(&mut self) {
        // for i in 0..1024 {
        //     self.free_from_here[i] = true;
        // }
    }
}

fn printf(msg: &str) {
    unsafe { libc::printf(msg.as_ptr() as *const libc::c_char) };
}

unsafe impl<const T: usize> GlobalAlloc for MyAllocator<T> {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        for i in 0..T {
            let mut found = true;
            let mut ran = false;
            for j in 0..layout.size() {
                ran = true;
                let pos = i + j;
                let in_bounds = pos < T;
                // if in_bounds {
                //     libc::printf("in bounds: %d\n\0".as_ptr() as *const libc::c_char);
                // }
                if !in_bounds || !self.free_from_here[pos] {
                    found = false;
                    break;
                }
            }

            if found {
                // printf("found!!\n\0");
            }

            if ran {
                // printf("ran!!\n\0");
            }

            if !found || !ran {
                // printf("no found nor ran, skip!!\n\0");
                continue;
            }

            let base: *mut u8 = self.data.as_ptr() as *mut u8;

            let free_base = self.free_from_here.as_ptr() as *mut bool;
            for j in 0..layout.size() {
                let pos = (i + j) as isize;
                println!(
                    "Setting to false at: {}, i: {}, j: {}, size: {}",
                    pos,
                    i,
                    j,
                    layout.size()
                );
                *free_base.offset(pos) = false;
            }

            return base.offset(i as isize);
        }
        // libc::printf("ret null\n\0".as_ptr() as *const libc::c_char);
        std::ptr::null_mut()
    }
    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        std::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {}
    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        std::ptr::null_mut()
    }
}

pub fn main() {
    // unsafe { GLOBAL.init() };
    // let letters: Vec<&str> = vec!["a", "b", "cd", "efg", "h"];
    // let joined: String = letters
    //     .into_iter()
    //     .map(|s| String::from(s).to_uppercase())
    //     .collect();
    // println!("Result: {}", joined);
}

#[cfg(test)]
mod tests {
    use std::alloc::{GlobalAlloc, Layout};

    use super::MyAllocator;

    #[test]
    fn test_my_allocator_succeeds() {
        let mut allocator = MyAllocator::<1024>::new();
        allocator.init();
        let base_addr: *const u8 = allocator.data.as_ptr();
        let layout4 = Layout::from_size_align(4, 4).unwrap();
        let layout8 = Layout::from_size_align(8, 4).unwrap();
        let layout1 = Layout::from_size_align(1, 4).unwrap();

        let alloc = |layout: Layout| unsafe {
            let ptr = allocator.alloc(layout);
            ptr.offset_from(base_addr)
        };

        assert_eq!(0, alloc(layout4));
        assert_eq!(4, alloc(layout8));
        assert_eq!(12, alloc(layout1));
        assert_eq!(13, alloc(layout1));
        assert_eq!(14, alloc(layout1));
        assert_eq!(15, alloc(layout8));
        assert_eq!(23, alloc(layout8));
    }

    #[test]
    fn test_my_allocator_out_of_memory() {
        let mut allocator = MyAllocator::<7>::new();
        allocator.init();
        let base_addr: *const u8 = allocator.data.as_ptr();
        let layout4 = Layout::from_size_align(4, 4).unwrap();
        let layout8 = Layout::from_size_align(8, 4).unwrap();
        let layout3 = Layout::from_size_align(3, 4).unwrap();

        let alloc = |layout: Layout| unsafe {
            let ptr = allocator.alloc(layout);
            ptr.offset_from(base_addr)
        };

        assert_eq!(0, alloc(layout4));
        assert_eq!(std::ptr::null(), unsafe { allocator.alloc(layout8) });
        assert_eq!(std::ptr::null(), unsafe { allocator.alloc(layout4) });
        assert_eq!(4, alloc(layout3));
    }
}
