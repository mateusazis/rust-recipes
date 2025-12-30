async fn double(n: i32) -> i32 {
    println!(
        "Making the double of: {} from thread {}",
        n,
        std::thread::current().name().unwrap()
    );
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    n * 2
}

fn double_blocking(n: i32) -> i32 {
    println!(
        "[Slow] Making the double of: {} from thread {}",
        n,
        std::thread::current().name().unwrap()
    );
    std::thread::sleep(std::time::Duration::from_secs(1));
    n * 2
}

async fn sum_async() -> i32 {
    println!("v1");
    let v1 = tokio::task::spawn(async { double(10).await });
    println!("v2");
    let v2 = tokio::task::spawn(async { double(3).await });
    println!("v3");
    let v3 = tokio::task::spawn_blocking(|| double_blocking(1));

    let futures = vec![v1, v2, v3];
    let values = futures::future::join_all(futures).await;
    values.into_iter().map(|i| i.unwrap()).sum()
}

async fn main_async() {
    let result = sum_async().await;
    assert_eq!(result, 28);
    println!("Result: {}", result);
    println!("Sleeping...");
}

// Works

// #[tokio::main]
// async fn main() {
//     let fut = main_async().await;
// }

// Also works

fn main() {
    let thread_counter = std::sync::Mutex::new(0);
    tokio::runtime::Builder::new_multi_thread()
        .thread_name_fn(move || {
            let mut locked = thread_counter.lock().unwrap();
            let thread_number = locked.clone();
            *locked += 1;
            format!("foo_bar_{}", thread_number)
        })
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            main_async().await;
        });
}
