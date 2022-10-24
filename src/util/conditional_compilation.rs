fn get_secret_of_life() -> Option<i32> {
  // let mut secret_known = false;

  #[cfg(feature = "secret_of_life_known")]
  return Some(42);

  #[cfg(not(feature = "secret_of_life_known"))]
  return None;
}

pub fn main() {
  println!("The secret of life is: {:?}", get_secret_of_life())
}
