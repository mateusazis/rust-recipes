pub struct HelloProducer {}

impl HelloProducer {
    pub fn get_hello(&self) -> &'static str {
        "hello world from library"
    }
}
