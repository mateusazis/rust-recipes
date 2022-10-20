
#[link(name = "mylib")]
extern {
  fn toUpper(input : *mut u8);
}

pub fn main() {
  let mut test_string = String::from("this IS A test");
  unsafe {
    toUpper(test_string.as_mut_str().as_ptr() as *mut u8);
  }
  println!("Result: {}", test_string);
}
