use futures::future::{join, BoxFuture};
use futures::task::{waker_ref, ArcWake};
use futures::{Future, FutureExt};
use std::fmt::Display;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

struct DelayedResult {
    current_call_count: Mutex<i32>,
    required_call_count: i32,
    result: i32,
}

impl DelayedResult {
    fn new(required_call_count: i32, result: i32) -> DelayedResult {
        DelayedResult {
            current_call_count: Mutex::new(0),
            required_call_count,
            result,
        }
    }
}

impl Future for DelayedResult {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let lock = (*self).current_call_count.lock();
        let mut wrapper = lock.unwrap();
        let new_value = *wrapper + 1;
        *wrapper = new_value;
        std::mem::drop(wrapper);

        println!("Count for {} is: {}", (*self).result, new_value);
        if new_value >= self.required_call_count {
            return Poll::Ready(self.result);
        }
        cx.waker().clone().wake();
        Poll::Pending
    }
}

struct BlockingTaskFuture<F, T>
where
    F: Fn() -> T + Send + Sync + Copy + 'static,
    T: Send + Sync + 'static + Display + Copy,
{
    task: F,
    result: Arc<Mutex<Option<T>>>,
}

impl<F, T> Future for BlockingTaskFuture<F, T>
where
    F: Fn() -> T + Send + Sync + Copy + 'static,
    T: Send + Sync + 'static + Display + Copy,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(ret) = *self.result.lock().unwrap() {
            return Poll::Ready(ret);
        }

        let task = (*self).task;

        let res = self.result.clone();
        let w = cx.waker().clone();

        std::thread::Builder::new()
            .name(String::from("blocking thread"))
            .spawn(move || {
                let ret = (task)();
                println!("Ret: {}", ret);
                *res.lock().unwrap() = Some(ret);
                w.wake_by_ref();
            })
            .expect("should spawn thread");
        Poll::Pending
    }
}

async fn run_blocking<F, T>(func: F) -> T
where
    F: Fn() -> T + Send + Sync + Copy + 'static,
    T: Send + Sync + 'static + Display + Copy,
{
    BlockingTaskFuture {
        task: func,
        result: Arc::new(Mutex::new(None)),
    }
    .await
}

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
        let (sender, receiver) = sync_channel::<Arc<Task>>(8096);
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

    fn send(&self, future: BoxFuture<'static, i32>) {
        let task = Arc::new(Task {
            future: Mutex::new(future.boxed()),
            sender: self.sender.clone(),
            done: Mutex::new(false),
        });
        self.sender.send(task).expect("should send");
    }
}

async fn build_simple_future() -> i32 {
    let future_a = DelayedResult::new(2, 10);
    let future_b = DelayedResult::new(4, 14);
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

pub fn main() {
    let executor = Executor::new(Duration::from_millis(500));

    executor.send(
        run_blocking(|| {
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
        })
        .boxed(),
    );

    executor.send(DelayedResult::new(5, 42).boxed());
    // executor.send(DelayedResult::new(10, -3).boxed());
    // executor.send(build_simple_future().boxed());
    executor.join();
}
