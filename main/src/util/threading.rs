struct Wrapper(i32);

impl Wrapper {
    fn print(&self) {
        println!("My value is {}", self.0);
    }
}

pub fn main() {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let wrapper = std::sync::Arc::new(Wrapper(42));
    let t1 = std::thread::spawn(move || {
        loop {
            sender.send(wrapper.clone()).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
    // t1.join().unwrap();
    let t2 = std::thread::spawn(move || {
        loop {
            receiver.recv().expect("should recv").print();
        }
    });
    t2.join().unwrap();
    t1.join().unwrap();
}
