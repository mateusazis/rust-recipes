use std::io::Read;

enum ResultType {
    Success(i32),
    Failure1,
    Failure2,
}

#[derive(Debug)]
struct CodedFailure(i32);

impl std::error::Error for CodedFailure {}

impl std::fmt::Display for CodedFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FAIL({})", self.0)
    }
}

fn compute_something(result_type: ResultType) -> Result<i32, Box<dyn std::error::Error>> {
    let mut reader = std::io::BufReader::new(std::fs::File::open("/dev/random")?);
    println!("Here are 10 random numbers:");
    let mut buf = [0u8; 1];
    for _ in 0..10 {
        reader.read(&mut buf)?;
        println!("Got value: {}", buf[0]);
    }
    std::mem::drop(reader);
    match result_type {
        ResultType::Success(x) => Ok(x),
        ResultType::Failure1 => Err(Box::new(CodedFailure(11))),
        ResultType::Failure2 => Err(Box::new(CodedFailure(42))),
    }
}

pub fn main() {
    // let result_type = ResultType::Success(10);
    // let result_type = ResultType::Failure1;
    let result_type = ResultType::Failure2;
    let result = compute_something(result_type);
    match result {
        Ok(x) => println!("The result was: {}", x),
        Err(msg) => println!("Failed with error: {}", msg),
    };
}
