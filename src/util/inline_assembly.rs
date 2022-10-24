use std::arch::asm;

fn sum(a : i32, b : i32) -> i32 {
  let mut result : i32;

  // Using Mac's aarch64 instruction set.
  // See: https://developer.arm.com/documentation/102374/0100/Overview
  unsafe {
    asm!("mov w0, {1:w}",
      "mov w1, {2:w}",
      "add w0, w0, w1",
      "mov {0:w}, w0",
      out(reg) result,
      in(reg) a,
      in(reg) b,
    );
  }
  result
}

fn array_multiply(numbers: &mut [i32], multiplier: i32) {
  // for i in 0..numbers.len() {
    unsafe {
      let ptr = numbers.as_ptr();
      asm!(
        "mov x0, {0}",
        "ldr w0, x0",
        "mov w1, {1:w}",
        "mul w0, w0, w1",
        "str w0, x0",
        in(reg) ptr,
        in(reg) multiplier,
      );
    }
  // }
}


pub fn main() {
  let a = 9;
  let b = 4;
  let result = sum(a, b);
  println!("The sum of {} and {} is {}", a, b, result);
  unsafe {
    let (mut my_array, _, _) = vec![4, 9, 13, 27].into_raw_parts();
    println!("Original array: {:?}", my_array);
    let len = my_array.len();
    array_multiply(&mut my_array[0..len], 3);
    println!("After multiplication: {:?}" , my_array);
  }
}
