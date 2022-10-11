use std::io::Error;

use async_std::io::ReadExt;

async fn main_async() -> (){
  let task1 = 
    async_std::task::spawn(read_file("/dev/random", "one"));
  let task2 = async_std::task::spawn(read_file("/dev/random", "two"));
  
  let (r0, r1) = futures::join!(task1, task2);
  r0.and(r1).expect("expected results");
}

async fn read_file(path : &str, diff : &str) -> Result<(), async_std::io::Error> {
  let file = async_std::fs::File::open(path).await?;
  let mut reader = async_std::io::BufReader::new(file);
  let mut buf = [0u8];
  while reader.read(&mut buf).await? > 0 {
    println!("[{}] From file {}: {}", diff, path, buf[0]);
  }
  return Ok(())
}

pub fn main() {
  async_std::task::block_on(main_async())
}
