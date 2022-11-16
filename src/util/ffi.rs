use std::ffi::CString;

#[link(name = "mylib")]
extern "C" {
    fn toUpper(input: *mut libc::c_char) -> i32;
}

pub fn main() {
    let test_string = CString::new("this IS A test").unwrap();
    let ptr = test_string.as_ptr() as *mut libc::c_char;
    println!("[Rust] send string at 0x{:x}", ptr as u64);
    let length = unsafe { toUpper(ptr) };
    println!(
        "Result: '{}', with length: {}",
        String::from_utf8_lossy(test_string.as_bytes()),
        length
    );
}
