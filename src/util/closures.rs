#[derive(Debug)]
struct IntWrapper(i32);

fn modify_the_wrapper<F>(n : &IntWrapper, f : F) -> i32
where F: FnOnce(IntWrapper) -> i32 {
  let w2 = IntWrapper(n.0*2);
  f(w2)
}

fn modify_the_double<F>(n : i32, f : F) -> i32
where F: FnOnce(i32) -> i32 {
  f(n*2)
}

pub fn main() {
  let value = 42;
  let modified = modify_the_double(value, |x| x+1);
  println!("{} was modified to {}", value, modified);

  let wrapper = IntWrapper(42);
  let modified = modify_the_wrapper(&wrapper, move |w| w.0+wrapper.0);
  println!("Wrapper {:?} was modified to {:?}", wrapper, modified);
}
