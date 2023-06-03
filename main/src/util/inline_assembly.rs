use std::arch::asm;

fn sum(a: i32, b: i32) -> i32 {
    let result: i32;

    // Using Mac's aarch64 instruction set.
    // See: https://developer.arm.com/documentation/102374/0100/Overview
    #[cfg(target_arch = "aarch64")]
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

    #[cfg(target_arch = "x86_64")]
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

            #[cfg(target_arch = "aarch64")]
            // working:
            // order: DEST, OPERAND1, OPERAND2
            asm!(
              "add x0, {0:x}, {1:x}",
              "ldr w30, [x0]",
              "mul w30, w30, {2:w}",
              "str w30, [x0]",
              in(reg) ptr_base,
              in(reg) offset,
              in(reg) multiplier,
            );

            #[cfg(target_arch = "x86_64")]
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

fn write_via_syscall(mut message: String) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let ptr = message.as_mut_str().as_mut_ptr();
        let len = message.len();
        let mut out: i32;

        asm!(
          "mov rax, 1",
          "mov rdi, 1",
          "mov rsi, {0:r}",
          "mov rdx, {2:r}",
          "syscall",
          "mov {1:r}, rax",
          in(reg) ptr,
          out(reg) out,
          in(reg) len,
        );
        let r = libc::strerror(-out);
        libc::printf(r);
    }
}

fn exit(code: i32) {
    // exit via syscall
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
          "mov rax, 60",
          "mov rdi, {0:r}",
          "syscall",
          in(reg) code,
        );
    }
}

fn get_pid() {
    let my_pid = syscall0(39);
    println!("My pid is {}", my_pid);
}

fn syscall0(number: i32) -> i32 {
    #[cfg(target_arch = "aarch64")]
    return 0;

    // For the x86_64 registers: https://stackoverflow.com/a/2538212
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut ret: i32;
        asm!(
          "mov rax, {0:r}",
          "syscall",
          "mov {1:r}, rax",
          in(reg) number,
          out(reg) ret,
        );
        return ret;
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
    println!("After multiplication: {:?}", my_array);

    get_pid();
    write_via_syscall(String::from("hello world\n"));

    // std::thread::sleep(std::time::Duration::from_secs(60));
    exit(37);
}
