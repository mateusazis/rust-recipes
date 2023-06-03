trait Base {
    fn get_v(&self) -> i32;
}

struct Derived1 {}

impl Base for Derived1 {
    fn get_v(&self) -> i32 {
        42
    }
}

struct Derived2 {}

impl Base for Derived2 {
    fn get_v(&self) -> i32 {
        89
    }
}

fn print_without_template(b: &dyn Base) {
    let addr = print_without_template as *const u8;
    println!("Func @ 0x{:x}, value: {}", addr as u64, b.get_v());
}

// Should work the same as print_with_template
fn print_with_impl(b: &impl Base) {
    // We cannot reference the actual directly because it is,
    // by definition, anonymous.
    let f1: fn(b: &Derived1) -> () = print_with_impl;
    let addr1 = f1 as *const u8;

    let f2: fn(b: &Derived2) -> () = print_with_impl;
    let addr2 = f2 as *const u8;

    let addr = format!("0x{:x} | 0x{:x}", addr1 as u64, addr2 as u64);
    println!("Func @ {}, value: {}", addr, b.get_v());
}

fn print_with_template<T: Base>(b: &T) {
    let addr = print_with_template::<T> as *const u8;
    println!("Func @ 0x{:x}, value: {}", addr as u64, b.get_v());
}

trait Getter {
    fn get_integer() -> i32;
}

impl Getter for i32 {
    fn get_integer() -> i32 {
        40
    }
}
impl Getter for f32 {
    fn get_integer() -> i32 {
        226
    }
}

fn print_integer<T: Getter>() {
    let v: i32 = T::get_integer();
    println!("Integer value: {}", v);
}

pub fn main() {
    let d1 = Derived1 {};
    let d2 = Derived2 {};

    println!("With 'dyn':");
    print_without_template(&d1);
    print_without_template(&d2);

    println!("\nWith template:");
    print_with_template(&d1);
    print_with_template(&d2);

    println!("\nWith 'impl':");
    print_with_impl(&d1);
    print_with_impl(&d2);

    print_integer::<i32>();
    print_integer::<f32>();
}
