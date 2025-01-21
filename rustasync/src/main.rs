async fn double(n: i32) -> i32 {
    n * 2
}

async fn sum_async() -> i32 {
    let v1 = double(10);
    let v2 = double(4);
    v1.await + v2.await
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
    tokio::runtime::Builder::new_current_thread()
        .thread_name("foo_bar")
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            main_async().await;
        });
}
