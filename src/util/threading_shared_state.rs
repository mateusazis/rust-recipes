use std::{
    sync::{Arc, RwLock},
    thread::JoinHandle,
};

struct Wrapper(i32);

impl Wrapper {
    fn inc(&mut self) {
        self.0 += 1;
    }
}

fn make_thread(wrapper: Arc<RwLock<Wrapper>>, thread_name: &str) -> JoinHandle<()> {
    let thread_name = String::from(thread_name);
    std::thread::spawn(move || loop {
        wrapper.write().unwrap().inc();
        println!("{}: {}", thread_name, wrapper.read().unwrap().0);
        std::thread::sleep(std::time::Duration::from_millis(500));
    })
}

pub fn main() {
    let wrapper = Arc::new(RwLock::new(Wrapper(0)));
    let t1 = make_thread(wrapper.clone(), "T1");
    let t2 = make_thread(wrapper, "T2");
    t2.join().unwrap();
    t1.join().unwrap();
}
