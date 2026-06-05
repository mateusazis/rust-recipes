mod executor;

async fn double(n: i32) -> i32 {
    println!(
        "[Non-blocking] Making the double of: {} from thread {}",
        n,
        std::thread::current().name().unwrap_or("main")
    );
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    n * 2
}

fn double_blocking(n: i32) -> i32 {
    println!(
        "[Blocking] Making the double of: {} from thread {}",
        n,
        std::thread::current().name().unwrap_or("main")
    );
    std::thread::sleep(std::time::Duration::from_secs(1));
    n * 2
}

async fn sum_async() -> i32 {
    println!("v1");
    let v1 = executor::spawn(async { double(10).await });
    println!("v2");
    let v2 = executor::spawn(async { double(3).await });
    println!("v3");
    let v3 = executor::spawn_blocking(|| double_blocking(1));

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

fn main() {
    let runtime1 = executor::Runtime::new(4);
    let runtime2 = executor::Runtime::new(1);

    println!("--- Running on Runtime 1 (4 threads) ---");
    let res1 = runtime1.block_on(async {
        main_async().await;
        28
    });
    assert_eq!(res1, 28);

    println!("\n--- Running on Runtime 2 (1 thread) ---");
    let res2 = runtime2.block_on(async {
        main_async().await;
        28
    });

    assert_eq!(res2, 28);
    println!("\nBoth runtimes ran successfully and returned correct results!");
}


