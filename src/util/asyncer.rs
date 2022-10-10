use async_std::io::ReadExt;

async fn main_async() {
  let task1 = 
    async_std::task::spawn(read_file("/dev/random", "one"));
  let task2 = async_std::task::spawn(read_file("/dev/random", "two"));
  
  futures::join!(task1, task2);
}

async fn read_file(path : &str, diff : &str) {
  let file = async_std::fs::File::open(path).await.expect(format!("should open file: {}", path).as_str());
  let mut reader = async_std::io::BufReader::new(file);
  let mut buf = [0u8];
  while reader.read(&mut buf).await.expect("should read one byte") > 0 {
    println!("[{}] From file {}: {}", diff, path, buf[0]);
  }
}

pub fn main() {
  async_std::task::block_on(main_async())
}
