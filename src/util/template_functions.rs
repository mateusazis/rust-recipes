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

fn print_with_template<T: Base>(b: &T) {
    let addr = print_with_template::<T> as *const u8;
    println!("Func @ 0x{:x}, value: {}", addr as u64, b.get_v());
}

pub fn main() {
    let d1 = Derived1 {};
    let d2 = Derived2 {};

    print_without_template(&d1);
    print_without_template(&d2);

    print_with_template(&d1);
    print_with_template(&d2);
}
