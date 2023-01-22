use another_library::hello_producer::HelloProducer;

fn main() {
    println!(
        "Running from another binary! Got from lib: '{}'",
        HelloProducer {}.get_hello()
    );
}
