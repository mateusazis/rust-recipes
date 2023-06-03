trait Multiplier {
    fn multiply(&self, a: i32, b: i32) -> i32;
}

struct SimpleMultipler {}

impl Multiplier for SimpleMultipler {
    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}
struct LoopMultipler {}

impl Multiplier for LoopMultipler {
    fn multiply(&self, a: i32, b: i32) -> i32 {
        let mut result = 0;
        for _ in 0..b {
            result += a;
        }
        result
    }
}

enum MultiplierType {
    Simple,
    Loop,
}

fn make_multiplier(t: MultiplierType) -> &'static dyn Multiplier {
    let m: &dyn Multiplier = match t {
        MultiplierType::Simple => &SimpleMultipler {},
        MultiplierType::Loop => &LoopMultipler {},
    };
    m
}

fn multiply(a: i32, b: i32, multiplier: &dyn Multiplier) -> (i32, &dyn Multiplier) {
    (multiplier.multiply(a, b), multiplier)
}

fn multiply_knowing_the_type(
    a: i32,
    b: i32,
    multiplier: &impl Multiplier,
) -> (i32, &impl Multiplier) {
    (multiplier.multiply(a, b), multiplier)
}

pub fn main() {
    let a = 10;
    let b = 6;
    let knowing = false;
    if knowing {
        let (result, _) = multiply_knowing_the_type(a, b, &SimpleMultipler {});
        println!("{}x{}={}", a, b, result);
    } else {
        let m = make_multiplier(MultiplierType::Loop);
        let (result, _) = multiply(a, b, m);
        println!("{}x{}={}", a, b, result);
    };
}
