use std::cell::{RefCell, Cell};
use std::rc::Rc;

struct Incrementer {
    value: Rc<Cell<i32>>,
}

impl Incrementer {
    fn inc(&self, inc: i32) {
        // changes internal state by moving values
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

fn run_cell() {
    let initial = 10;
    let value = Rc::new(Cell::new(initial));
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

    assert_eq!(value.get(), 75);
    assert_eq!(initial, 10);
}

struct RIncrementer {
    value: Rc<RefCell<i32>>,
}

impl RIncrementer {
    fn inc(&self, inc: i32) {
        // changes internal state by reference
        let old_value : i32 = *((*self.value).borrow());
        *self.value.borrow_mut() = old_value + inc;
    }

    fn print(&self) {
        println!("Value at Incrementer: {}", *((*self.value).borrow()))
    }
}

struct RMultiplier {
    value: Rc<RefCell<i32>>,
}

impl RMultiplier {
    fn mult(&self, multiplier: i32) {
        let old_value : i32 = *((*self.value).borrow());
        *self.value.borrow_mut() = old_value * multiplier;
    }

    fn print(&self) {
        println!("Value at Multiplier: {}", *((*self.value).borrow()))
    }
}

fn run_ref_cell() {
    let initial = Rc::new(RefCell::new(10));
    let inc = RIncrementer {
        value: initial.clone(),
    };
    let mult = RMultiplier {
        value: initial.clone(),
    };
    inc.inc(3);
    inc.inc(2);
    mult.mult(5);

    println!("Final value: {}", (*initial).borrow());
    inc.print();
    mult.print();

    assert_eq!(*(*initial).borrow(), 75);
}

pub fn main() {
    run_cell();
    run_ref_cell();
}