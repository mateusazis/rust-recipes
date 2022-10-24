use std::arch::asm;

fn sum(a : i32, b : i32) -> i32 {
  let result : i32;

  // Using Mac's aarch64 instruction set.
  // See: https://developer.arm.com/documentation/102374/0100/Overview
  #[cfg(target_arch="aarch64")]
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

  #[cfg(target_arch="x86_64")]
  unsafe {
    asm!(
      "mov {0:e}, {1:e}",
      "add {0:e}, {2:e}",
      out(reg) result,
      in(reg) a,
      in(reg) b,
    );
  }
  result
}

fn array_multiply(numbers: &mut [i32], multiplier: i32) {
  for i in 0..numbers.len() {
    unsafe {
      let mut i2 = (i * 4) as u64;
      let ptr_base = numbers.as_mut_ptr();
      let ptr = numbers.as_mut_ptr().offset(i as isize);
      #[cfg(target_arch="aarch64")]
      asm!(
        "mov x0, {0}",
        "ldr w0, x0",
        "mov w1, {1:w}",
        "mul w0, w0, w1",
        "str w0, x0",
        in(reg) ptr,
        in(reg) multiplier,
      );

      let mut tmp = 0i32;
      #[cfg(target_arch="x86_64")]
      // working:
      // order: DEST, SRC
      println!("pointer addr: {:?}", ptr_base as u64);
      asm!(
        "add {2}, {0}",
        "mov {3:e}, [{2}]",
        "imul {3:e}, {1:e}",
        "mov [{2}], {3:e}",
        in(reg) ptr_base,
        in(reg) multiplier,
        inout(reg) i2,
        out(reg) tmp,
      );
      println!("value of i2: {}, tmp: {}\n", i2, tmp);
    }
  }
}


pub fn main() {
  let a = 9;
  let b = 4;
  let result = sum(a, b);
  println!("The sum of {} and {} is {}", a, b, result);
  let mut my_array = [4, 9, 13, 27];
  println!("Original array: {:?}", my_array);
  let len = my_array.len();
  array_multiply(&mut my_array[0..len], 3);
  println!("After multiplication: {:?}" , my_array);
}
