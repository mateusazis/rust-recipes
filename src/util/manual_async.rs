use crate::util::manual_async_futures::{delayed, run_blocking_task};

use futures::future::{join, BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

const DEFAULT_CHANNEL_SIZE : usize = 8096;

struct Task {
    future: Mutex<BoxFuture<'static, i32>>,
    sender: SyncSender<Arc<Task>>,
    done: Mutex<bool>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.sender.send(cloned).expect("too many tasks queued");
    }
}

struct Executor {
    sender: SyncSender<Arc<Task>>,
    thread: JoinHandle<()>,
}

impl Executor {
    fn new(delay: Duration) -> Executor {
        let (sender, receiver) = sync_channel::<Arc<Task>>(DEFAULT_CHANNEL_SIZE);
        let thread = std::thread::Builder::new()
            .name(String::from("Executor"))
            .spawn(move || {
                while let Ok(task) = receiver.recv() {
                    let mut future_slot = task.future.lock().unwrap();
                    if *task.done.lock().unwrap() {
                        continue;
                    }
                    let waker = waker_ref(&task);
                    let ctx = &mut Context::from_waker(&*waker);
                    let execution_result = future_slot.as_mut().poll(ctx);
                    match execution_result {
                        Poll::Pending => {
                            sleep(delay);
                        }
                        Poll::Ready(val) => {
                            println!("Execution finished with: {}", val);
                            *task.done.lock().unwrap() = true;
                        }
                    }
                }
            })
            .expect("should spawn executor thread");
        Executor { sender, thread }
    }

    fn join(self) {
        std::mem::drop(self.sender);
        self.thread.join().expect("should join");
    }

    fn send(&self, future: impl FutureExt<Output = i32> + Send + 'static)
    {
        let task = Arc::new(Task {
            future: Mutex::new(future.boxed()),
            sender: self.sender.clone(),
            done: Mutex::new(false),
        });
        self.sender.send(task).expect("should send");
    }
}

async fn build_simple_future() -> i32 {
    let future_a = delayed(2, 10);
    let future_b = delayed(4, 14);
    // let future_b = async {
    //   33
    // };

    // let a = future_a.await;
    // let b= future_b.await;

    let (a, b) = join(future_a, future_b).await;

    println!("Got results, sleeping for 2 secs...");
    async_std::task::sleep(Duration::from_secs(2)).await;
    // let (a) = join(future_a).await;
    // a+1

    a + b + 1
}

fn do_blocking_work() -> i32 {
    let mut a = 23;
    println!("Pre-sleep...");
    let mut count = 0u64;
    for _ in 0..1_000_000_000u64 {
        count += 1;
    }
    // std::thread::sleep(Duration::from_secs(15));
    a += 100;
    println!("Post-sleep...");
    a
}

pub fn main() {
    let executor = Executor::new(Duration::from_millis(500));

    executor.send(
        async {
            let res1 = delayed(10, 3);
            println!("Res1: {}", res1.await);
            run_blocking_task(|| {
                let blocking_result = do_blocking_work();
                println!("Blocking result: {}", blocking_result);
                0
            })
            .await;
            let res2 = delayed(2, 4);
            println!("Res2: {}", res2.await);
            0
        }
    );

    let f = delayed(5, 42);
    executor.send(f);
    // executor.send(DelayedResult::new(10, -3).boxed());
    // executor.send(build_simple_future().boxed());
    executor.join();
}
