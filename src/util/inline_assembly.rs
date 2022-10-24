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
  let ptr_base = numbers.as_mut_ptr();
  for i in 0..numbers.len() {
    unsafe {
      let offset = (i * std::mem::size_of::<i32>()) as u64;

      let mut temp = 0i32;

      #[cfg(target_arch="aarch64")]
      asm!(
        "mov x0, {0:x}",
        "add x0, x0, {1:x}",
        "ldr {3:w}, [x0]",
        "mov w1, {2:w}",
        "mul {3:w}, {3:w}, w1",
        "str {3:w}, [x0]",
        in(reg) ptr_base,
        in(reg) offset,
        in(reg) multiplier,
        inout(reg) temp,
      );

      println!("Temp ended as: {}", temp);

      #[cfg(target_arch="x86_64")]
      // working:
      // order: DEST, SRC
      asm!(
        "add {1}, {0}",
        "mov ebx, [{1}]",
        "imul ebx, {2:e}",
        "mov [{1}], ebx",
        in(reg) ptr_base,
        in(reg) offset,
        in(reg) multiplier,
      );
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
