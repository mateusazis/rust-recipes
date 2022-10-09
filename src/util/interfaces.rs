trait Multiplier {
  fn multiply(&self, v : i32) -> i32;
}

struct ConstantMultiplier{multiplier : i32}

impl Multiplier for ConstantMultiplier {
  fn multiply(&self, v : i32) -> i32 {
      self.multiplier*v
  }
}

fn multiply(value : i32, m : &impl Multiplier) -> i32 {
  m.multiply(value)
}

fn run_loop(m : &impl Multiplier) {
  for i in 0..10 {
    println!("{} when multiplied is {}", i, multiply(i, m))
  }
  println!("")
}

pub fn interfaces_main()  {
  let m = ConstantMultiplier{multiplier: 3};
  run_loop(&m);

  run_loop(&m);

  let m = ConstantMultiplier{multiplier: 15};
  run_loop(&m)
}
