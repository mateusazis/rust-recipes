use std::cell::Cell;
use std::rc::Rc;

struct Incrementer {
    value: Rc<Cell<i32>>,
}

impl Incrementer {
    fn inc(&self, inc: i32) {
        let old_value = self.value.get();
        self.value.set(old_value + inc);
    }

    fn print(&self) {
        println!("Value at Incrementer: {}", self.value.get())
    }
}

struct Multiplier {
    value: Rc<Cell<i32>>,
}

impl Multiplier {
    fn mult(&self, multiplier: i32) {
        let old_value = self.value.get();
        self.value.set(old_value * multiplier);
    }

    fn print(&self) {
        println!("Value at Multiplier: {}", self.value.get())
    }
}

pub fn main() {
    let value = Rc::new(Cell::new(10));
    let inc = Incrementer {
        value: value.clone(),
    };
    let mult = Multiplier {
        value: value.clone(),
    };
    inc.inc(3);
    inc.inc(2);
    mult.mult(5);

    println!("Final value: {}", value.get());
    inc.print();
    mult.print();
}
