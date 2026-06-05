use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

enum Message {
    RunTask(Arc<Task>),
    Shutdown,
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
    task_sender: SyncSender<Message>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let cloned = self.clone();
        let _ = self.task_sender.send(Message::RunTask(cloned));
    }
}

thread_local! {
    static CURRENT_SENDER: std::cell::RefCell<Option<SyncSender<Message>>> = std::cell::RefCell::new(None);
}

pub struct JoinHandle<R> {
    shared_state: Arc<Mutex<JoinState<R>>>,
}

struct JoinState<R> {
    result: Option<R>,
    waker: Option<Waker>,
}

#[derive(Debug)]
pub struct JoinError;

impl std::fmt::Display for JoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JoinError")
    }
}

impl std::error::Error for JoinError {}

impl<R> Future for JoinHandle<R> {
    type Output = Result<R, JoinError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();
        if let Some(res) = state.result.take() {
            Poll::Ready(Ok(res))
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn spawn_on<F, R>(future: F, sender: &SyncSender<Message>) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let shared_state = Arc::new(Mutex::new(JoinState {
        result: None,
        waker: None,
    }));

    let state_clone = shared_state.clone();
    let task_future = async move {
        let result = future.await;
        let mut state = state_clone.lock().unwrap();
        state.result = Some(result);
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    };

    let task = Arc::new(Task {
        future: Mutex::new(Some(Box::pin(task_future))),
        task_sender: sender.clone(),
    });

    let _ = sender.send(Message::RunTask(task));

    JoinHandle { shared_state }
}

pub fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let sender = CURRENT_SENDER.with(|cell| {
        cell.borrow()
            .clone()
            .expect("spawn called outside of an executor context")
    });

    spawn_on(future, &sender)
}

pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let shared_state = Arc::new(Mutex::new(JoinState {
        result: None,
        waker: None,
    }));

    let state_clone = shared_state.clone();
    std::thread::Builder::new()
        .name("blocking_thread".to_string())
        .spawn(move || {
            let result = f();
            let mut state = state_clone.lock().unwrap();
            state.result = Some(result);
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        })
        .unwrap();

    JoinHandle { shared_state }
}

struct Executor {
    task_receiver: Receiver<Message>,
}

impl Executor {
    fn new() -> (Self, SyncSender<Message>) {
        let (sender, receiver) = sync_channel(10_000);
        (
            Executor {
                task_receiver: receiver,
            },
            sender,
        )
    }

    fn run(&self, sender: SyncSender<Message>) {
        CURRENT_SENDER.with(|cell| {
            *cell.borrow_mut() = Some(sender);
        });

        while let Ok(msg) = self.task_receiver.recv() {
            match msg {
                Message::RunTask(task) => {
                    let mut future_slot = task.future.lock().unwrap();
                    if let Some(mut future) = future_slot.take() {
                        let waker = Waker::from(task.clone());
                        let mut context = Context::from_waker(&waker);
                        if future.as_mut().poll(&mut context).is_pending() {
                            *future_slot = Some(future);
                        }
                    }
                }
                Message::Shutdown => {
                    break;
                }
            }
        }

        CURRENT_SENDER.with(|cell| {
            *cell.borrow_mut() = None;
        });
    }
}

pub struct Runtime {
    task_sender: SyncSender<Message>,
    executor_thread: Option<std::thread::JoinHandle<()>>,
}

impl Runtime {
    pub fn new() -> Self {
        let (executor, sender) = Executor::new();
        let sender_clone = sender.clone();

        let executor_thread = std::thread::Builder::new()
            .name("executor_thread".to_string())
            .spawn(move || {
                executor.run(sender);
            })
            .unwrap();

        Runtime {
            task_sender: sender_clone,
            executor_thread: Some(executor_thread),
        }
    }

    pub fn block_on<F, R>(&self, future: F) -> R
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();

        spawn_on(
            async move {
                let res = future.await;
                let _ = tx.send(res);
            },
            &self.task_sender,
        );

        rx.recv().unwrap()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        let _ = self.task_sender.send(Message::Shutdown);
        if let Some(thread) = self.executor_thread.take() {
            let _ = thread.join();
        }
    }
}
