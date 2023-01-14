use async_std::future;
use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use futures::{Future, FutureExt};
use std::pin::Pin;
use std::process::Output;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

struct IntWrapper(i32);

struct DelayedResult {
    current_call_count: Mutex<IntWrapper>,
    required_call_count: i32,
    result: i32,
}

impl DelayedResult {
    fn new(required_call_count: i32, result: i32) -> DelayedResult {
        DelayedResult {
            current_call_count: Mutex::new(IntWrapper(0)),
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
        let new_value = wrapper.0 + 1;
        wrapper.0 = new_value;
        std::mem::drop(wrapper);

        if new_value >= self.required_call_count {
            return Poll::Ready(self.result);
        }
        cx.waker().clone().wake();
        Poll::Pending
    }
}

struct Executor {
    sender: SyncSender<i32>,
    thread: JoinHandle<()>,
}

impl Executor {
    fn new(delay: Duration) -> Executor {
        let (sender, receiver) = sync_channel(8096);
        let thread = spawn(move || {
            while let Ok(val) = receiver.recv() {
                println!("Received: {}", val);
                sleep(delay);
            }
        });
        Executor { sender, thread }
    }

    fn join(self) {
        std::mem::drop(self.sender);
        self.thread.join().expect("should join");
    }
}

struct Task {
    future: Mutex<BoxFuture<'static, i32>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self.sender.send(cloned).expect("too many tasks queued");
    }
}

struct Executor2 {
    sender: SyncSender<Arc<Task>>,
    thread: JoinHandle<()>,
}

impl Executor2 {
    fn new(delay: Duration) -> Executor2 {
        let (sender, receiver) = sync_channel::<Arc<Task>>(8096);
        let thread = spawn(move || {
            while let Ok(task) = receiver.recv() {
                let mut future_slot = task.future.lock().unwrap();
                let waker = waker_ref(&task);
                let ctx = &mut Context::from_waker(&*waker);
                let execution_result = future_slot.as_mut().poll(ctx);
                match execution_result {
                    Poll::Pending => {
                        println!("Received pending, sleeping...");
                        sleep(delay);
                    }
                    Poll::Ready(val) => {
                        println!("Execution finished with: {}", val);
                        return;
                    }
                }
            }
        });
        Executor2 { sender, thread }
    }

    fn join(self) {
        std::mem::drop(self.sender);
        self.thread.join().expect("should join");
    }
}

pub fn main() {
    // let executor = Executor::new(Duration::from_millis(500));
    // executor.sender.send(10).unwrap();
    // executor.sender.send(20).unwrap();
    // executor.sender.send(30).unwrap();
    // executor.join();

    let executor2 = Executor2::new(Duration::from_millis(500));
    let fut = DelayedResult::new(5, 42);
    let task = Arc::new(Task {
        future: Mutex::new(fut.boxed()),
        sender: executor2.sender.clone(),
    });
    executor2.sender.send(task).expect("should send");
    executor2.join();

    // let mut val_future : Box<dyn Future<Output = i32>> = Box::new(DelayedResult::new(5, 42));

    // val_future.poll();
    // let val = futures::executor::block_on(val_future);
    // println!("Hello, got: {}", val);
}
