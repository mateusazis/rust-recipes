use futures::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};

struct DelayedResult {
    current_call_count: Mutex<i32>,
    required_call_count: i32,
    result: i32,
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

pub async fn delayed(required_call_count: i32, result: i32) -> i32 {
    DelayedResult {
        current_call_count: Mutex::new(0),
        required_call_count,
        result,
    }
    .await
}

struct BlockingTaskFuture<F, T>
where
    F: FnOnce() -> T + Send + Sync + Copy + 'static,
    T: Send + Sync + Copy + 'static,
{
    task: F,
    result: Arc<Mutex<Option<T>>>,
}

impl<F, T> Future for BlockingTaskFuture<F, T>
where
    F: FnOnce() -> T + Send + Sync + Copy + 'static,
    T: Send + Sync + Copy + 'static,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(ret) = *self.result.lock().unwrap() {
            return Poll::Ready(ret);
        }

        let task = (*self).task;

        let result_mutex = self.result.clone();
        let waker = cx.waker().clone();

        std::thread::Builder::new()
            .name(String::from("blocking thread"))
            .spawn(move || {
                let ret = (task)();
                *result_mutex.lock().unwrap() = Some(ret);
                waker.wake_by_ref();
            })
            .expect("should spawn thread");
        Poll::Pending
    }
}

pub async fn run_blocking_task<F, T>(func: F) -> T
where
    F: FnOnce() -> T + Send + Sync + Copy + 'static,
    T: Send + Sync + Copy + 'static,
{
    BlockingTaskFuture {
        task: func,
        result: Arc::new(Mutex::new(None)),
    }
    .await
}
