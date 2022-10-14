trait SummerIFace{
  type T;

  fn add(&mut self, value : Self::T);

  fn sum(&self) -> i32;
}

struct Summer {
  values : [Option<Box<dyn Number>>; 100],
  count : usize,
}

impl Summer {
  fn new()  -> Summer {
    const opt = None;
    Summer{values: [opt; 100], count: 0}
  }
}

impl <T: ?Sized> Number for Box<T> {
  fn get_value(&self) -> i32 {
      self.get_value()
  }
}

impl SummerIFace for Summer {
  type T = Box<dyn Number>;

  fn add(&mut self, value : Self::T) {
    self.values[self.count] = Some(value);
    self.count += 1;
  }

  fn sum(&self) -> i32 {
    let mut result = 0;
    let values = &self.values;
    for value in values {
      if let Some(boxedValue) = value {
        result += boxedValue.get_value()
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
  let x = Box::new(Direct(10));
  summer.add(x);
  // summer.add(&Direct(20));
  // summer.add(&Half(4));
  // summer.add(&Direct(30));
  // summer.add(&Half(14));

  println!("The sum is {}", summer.sum())
}
