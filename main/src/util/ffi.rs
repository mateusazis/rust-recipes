use std::ffi::CString;

#[link(name = "mylib")]
extern "C" {
    fn toUpper(input: *mut std::ffi::c_char) -> i32;
}

#[repr(C)]
#[derive(Debug)]
pub struct ResultString {
    pub str: [libc::c_char; 1024],
}

#[link(name = "mylib")]
extern "C" {
    fn toUpper2(input: *const std::ffi::c_char, result: *mut ResultString);
}

pub fn main() {
    let rust_str = "this IS A test";
    let test_string = CString::new(rust_str).unwrap();
    let ptr = test_string.as_ptr() as *mut libc::c_char;
    println!("[Rust] send string '{}' at 0x{:x}", rust_str, ptr as u64);
    let length = unsafe { toUpper(ptr) };
    println!(
        "[Rust] Result: '{}', with length: {}",
        String::from_utf8_lossy(test_string.as_bytes()),
        length
    );

    let mut result_struct: ResultString = unsafe { std::mem::zeroed() };
    unsafe { toUpper2(ptr, &mut result_struct) };
    let len = result_struct
        .str
        .into_iter()
        .position(|b| b == 0i8)
        .unwrap_or(0);
    let i8_slice = &result_struct.str[0..len];
    let u8_vec = i8_slice.into_iter().map(|c| *c as u8).collect();
    let std_string = String::from_utf8(u8_vec).unwrap();
    println!("[Rust] Result 2 (with struct): '{}'", std_string);
    println!("[Rust] final struct: '{:?}'", result_struct);
}
