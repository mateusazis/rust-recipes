use std::fmt::Display;

struct MyString {
    s: String,
    s_ptr: *const String,
}

impl MyString {
    fn new(s: &str) -> MyString {
        let mut result = MyString {
            s: String::from(s),
            s_ptr: std::ptr::null(),
        };
        result.s_ptr = &result.s;
        result
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            write!(
                f,
                "MyString: from value: '{}' (@0x{:x}), from ptr: '{}' (@0x{:x})",
                self.s,
                ((&self.s) as *const String) as u64,
                *self.s_ptr,
                self.s_ptr as u64,
            )
        }
    }
}

fn print_after_move(v: impl Display) {
    println!("After move: {}", v);
}

pub fn main() {
    let mut ms1 = MyString::new("test1");
    let mut ms2 = MyString::new("test2");
    println!("{}", ms1);
    println!("{}", ms2);

    println!("Swapping...");
    std::mem::swap(&mut ms1, &mut ms2);

    println!("{}", ms1);
    println!("{}", ms2);

    // succeeds
    assert_eq!(ms1.s, "test2");
    assert_eq!(ms2.s, "test1");

    // succeeds... but should it?
    unsafe {
        assert_eq!(*ms1.s_ptr, "test1");
        assert_eq!(*ms2.s_ptr, "test2");
    }
    print_after_move(ms2);

    println!();

    let p = MyString::new("test1");
    let pp = MyString::new("test2");
    let mut p1 = std::pin::Pin::new(&p);
    let mut p2 = std::pin::Pin::new(&pp);
    println!("P1: {}", p1);
    println!("P2: {}", p2);

    println!("Swapping...");
    std::mem::swap(&mut p1, &mut p2);

    println!("P1: {}", p1);
    println!("P2: {}", p2);
    print_after_move(p2);
}
