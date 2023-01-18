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
                "MyString: from value: '{}', from ptr: '{}'",
                self.s, *self.s_ptr
            )
        }
    }
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

    // fails
    unsafe {
        assert_eq!(*ms1.s_ptr, "test2");
        assert_eq!(*ms2.s_ptr, "test1");
    }
}
