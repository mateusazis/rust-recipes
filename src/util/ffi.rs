#[link(name = "mylib")]
extern "C" {
    fn toUpper(input: *mut u8) -> i32;
}

pub fn main() {
    let mut test_string = String::from("this IS A test");
    let length = unsafe { toUpper(test_string.as_mut_str().as_ptr() as *mut u8) };
    println!("Result: '{}', with length: {}", test_string, length);
}
