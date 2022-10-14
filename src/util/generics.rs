use std::ops::Deref;

trait SummerIFace{
  type T;

  fn add(&mut self, value : Self::T);

  fn sum(&self) -> i32;
}

const ARRAY_SIZE : usize = 100;

struct Summer<'a> {
  values : [Option<&'a dyn Number>; ARRAY_SIZE],
  count : usize,
}

impl <'a>  Summer<'a> {
  fn new()  -> Summer<'a> {
    Summer{values: [None; ARRAY_SIZE], count: 0}
  }
}

impl <T: ?Sized + Number> Number for Box<T> {
  fn get_value(&self) -> i32 {
      let x = self.deref();
      return x.get_value();
  }
}

impl<'a>  SummerIFace for Summer<'a>  {
  type T = &'a dyn Number;

  fn add(&mut self, value : Self::T) {
    self.values[self.count] = Some(value);
    self.count += 1;
  }

  fn sum(&self) -> i32 {
    let mut result = 0;
    let values = &self.values;
    for value in values {
      if let Some(boxed_value) = value {
        result += boxed_value.get_value()
      }
    }
    result
  }

// struct Summer<T> {
//   values : [Option<Box<T>>; 100],
//   count : usize,
// }

// impl <'a, T> Summer <T> where T : Number{
//   fn new()  -> Summer<T> {
//     Summer{values: [None; 100], count: 0}
//   }

//   fn add(&mut self, value : Box<T>) {
//     self.values[self.count] = Some(value);
//     self.count += 1;
//   }

//   fn sum(&self) -> i32 {
//     let mut result = 0;
//     let values = &self.values;
//     for value in values {
//       if let Some(boxedValue) = value {
//         result += boxedValue.get_value()
//       }
//     }
//     result
//   }
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
      self.0/2
  }
}


pub fn main() {
  // let mut summer : Summer<Box<dyn Number>> = Summer::new();
  let mut summer  = Summer::new();

  // summer.add(Direct(10));
  // let x = Box::new();
  summer.add(&Direct(10));
  summer.add(&Direct(20));
  summer.add(&Half(4));
  // summer.add(&Direct(30));
  // summer.add(&Half(14));

  println!("The sum is {}", summer.sum())
}
