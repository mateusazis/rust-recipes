trait Operator {
    fn compute(&self, a: i32, b: i32) -> i32;
}

struct Add {}

impl Operator for Add {
    fn compute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct Minus {
    add: Add,
}

impl Operator for Minus {
    fn compute(&self, a: i32, b: i32) -> i32 {
        self.add.compute(a, -b)
    }
}

impl Minus {
    fn new() -> Minus {
        Minus { add: Add {} }
    }
}

struct Multiply {}
impl Operator for Multiply {
    fn compute(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

struct Divide {}
impl Operator for Divide {
    fn compute(&self, a: i32, b: i32) -> i32 {
        a / b
    }
}

struct Calculator<'a> {
    operators: std::vec::Vec<&'a dyn Operator>,
    operands: std::vec::Vec<i32>,
    initial_value: i32,
}

impl<'a> Calculator<'a> {
    fn compute(&self) -> i32 {
        let mut v = self.initial_value;

        for i in 0..self.operators.len() {
            let operand = self.operands[i];
            v = self.operators[i].compute(v, operand);
        }

        v
    }

    fn push(&mut self, operand: i32, operator: &'a dyn Operator) {
        self.operands.push(operand);
        self.operators.push(operator);
    }

    fn new(initial_value: i32) -> Calculator<'a> {
        Calculator {
            initial_value,
            operands: std::vec::Vec::new(),
            operators: std::vec::Vec::new(),
        }
    }
}

impl<'a> Drop for Calculator<'a> {
    fn drop(&mut self) {
        println!("Destroy calculator that began with {}", self.initial_value)
    }
}

pub fn main() {
    let mut c = Calculator::new(10);

    c.push(2, &Add {});
    println!("Partial result is {}", c.compute());
    c.push(5, &Multiply {});
    println!("Partial result is {}", c.compute());
    c.push(3, &Divide {});
    println!("Partial result is {}", c.compute());

    println!("Result is {}", c.compute());
}
