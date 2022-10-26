use std::ops::Deref;

trait SummerIFace {
    type T;

    fn add(&mut self, number: Self::T);

    fn sum(&self) -> i32;
}

struct Summer<'a, const CAPACITY: usize> {
    numbers: [Option<&'a dyn Number>; CAPACITY],
    count: usize,
}

impl<'a, const CAPACITY: usize> Summer<'a, CAPACITY> {
    fn new() -> Summer<'a, CAPACITY> {
        Summer {
            numbers: [None; CAPACITY],
            count: 0,
        }
    }
}

impl<T: ?Sized + Number> Number for Box<T> {
    fn get_value(&self) -> i32 {
        let x = self.deref();
        x.get_value()
    }
}

impl<'a, const CAPACITY: usize> SummerIFace for Summer<'a, CAPACITY> {
    type T = &'a dyn Number;

    fn add(&mut self, number: Self::T) {
        self.numbers[self.count] = Some(number);
        self.count += 1;
    }

    fn sum(&self) -> i32 {
        let mut result = 0;
        let values = &self.numbers;
        for number in values.iter().flatten() {
            result += number.get_value()
        }
        result
    }
}

trait Number {
    fn get_value(&self) -> i32;
}

struct Direct(i32);

impl Number for Direct {
    fn get_value(&self) -> i32 {
        self.0
    }
}

struct Half(i32);

impl Number for Half {
    fn get_value(&self) -> i32 {
        self.0 / 2
    }
}

pub fn main() {
    let mut summer: Summer<100> = Summer::new();

    summer.add(&Direct(10));
    summer.add(&Direct(20));
    summer.add(&Half(4));

    println!("The sum is {}", summer.sum())
}
