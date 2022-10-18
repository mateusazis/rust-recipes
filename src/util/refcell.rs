use std::cell::RefCell;
use std::rc::Rc;

struct Incrementer{
  value: Rc<RefCell<i32>>
}

impl Incrementer {
  fn inc(&self, inc : i32){
    let old_value = *self.value.borrow();
    let new_value = old_value + inc;
    *self.value.borrow_mut() = new_value;
  }
}

struct Multiplier{
  value: Rc<RefCell<i32>>
}

impl Multiplier {
  fn mult(&self, multiplier : i32){
    let old_value = *self.value.borrow();
    let new_value = old_value * multiplier;
    *self.value.borrow_mut() = new_value;
  }
}

pub fn main() {
  let value = Rc::new(RefCell::new(10));
  let inc = Incrementer{value: value.clone()};
  let mult = Multiplier{value: value.clone()};
  inc.inc(3);
  inc.inc(2);
  mult.mult(5);

  println!("Final value: {}", value.borrow());
}
