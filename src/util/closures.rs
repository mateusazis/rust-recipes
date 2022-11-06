#[derive(Debug)]
struct IntWrapper(i32);

impl IntWrapper {
  fn plus_one(&self) -> i32{ 
    self.0+1
  }
}

impl Drop for IntWrapper {
  fn drop(&mut self) {
      println!("Dropping wrapper of {}", self.0);
  }
}

fn modify_the_wrapper<F>(n : &IntWrapper, f : F) -> i32
where F: FnOnce(IntWrapper) -> i32 {
  let w2 = IntWrapper(n.0*2);
  f(w2)
}

fn apply<F>(w : IntWrapper, f : F, count : i32) -> IntWrapper 
where F : Fn(IntWrapper) -> IntWrapper {
  let mut w2 = w;
  for _ in 0..count{ 
    w2 = f(w2);
  }
  w2
}

fn apply_ref<F>(w : &IntWrapper, f : F, count : i32) -> IntWrapper
where F : Fn(&IntWrapper) -> IntWrapper {
  let mut w2 = IntWrapper(w.0);
  for _ in 0..count{ 
    w2 = f(&w2);
  }
  w2
}

fn modify_the_double<F>(n : i32, f : F) -> i32
where F: FnOnce(i32) -> i32 {
  f(n*2)
}

pub fn main() {
  let value = 42;
  let modified = modify_the_double(value, |x| x+1);
  println!("{} was modified to {}", value, modified);

  {
    let wrapper = IntWrapper(42);
    let modified = modify_the_wrapper(&wrapper, move |w| w.0+wrapper.0);
    println!("Wrapper {:?} was modified to {:?}", wrapper, modified);
  }

  let wrapper = IntWrapper(10);
  let modified = apply_ref(&wrapper, |w| IntWrapper(w.0+1), 10);
  println!("Wrapper modified through ref to {:?}", modified);

  let plus_one = move |_ : IntWrapper| IntWrapper(wrapper.plus_one());
  
  println!("Wrapper modified to {:?}", apply(IntWrapper(0), plus_one, 10));
  // "move" does not let the calls below pass; wrapper was already moved in
  // println!("wrapper now is {:?}", wrapper);
  // println!("Wrapper modified to {:?}", apply(IntWrapper(0), plus_one, 10));
}
