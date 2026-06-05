use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

enum Message {
    RunTask(Arc<Task>),
    Shutdown,
}

struct TaskState {
    is_queued: bool,
    is_polling: bool,
    woken_during_poll: bool,
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
    task_sender: SyncSender<Message>,
    state: Mutex<TaskState>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let mut state = self.state.lock().unwrap();
        if state.is_polling {
            state.woken_during_poll = true;
        } else if !state.is_queued {
            state.is_queued = true;
            let cloned = self.clone();
            let _ = self.task_sender.send(Message::RunTask(cloned));
        }
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
        state: Mutex::new(TaskState {
            is_queued: true,
            is_polling: false,
            woken_during_poll: false,
        }),
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

#[derive(Clone)]
struct Executor {
    task_receiver: Arc<Mutex<Receiver<Message>>>,
}

impl Executor {
    fn new() -> (Self, SyncSender<Message>) {
        let (sender, receiver) = sync_channel(10_000);
        (
            Executor {
                task_receiver: Arc::new(Mutex::new(receiver)),
            },
            sender,
        )
    }

    fn run(&self, sender: SyncSender<Message>) {
        CURRENT_SENDER.with(|cell| {
            *cell.borrow_mut() = Some(sender);
        });

        loop {
            let msg = {
                let receiver = self.task_receiver.lock().unwrap();
                match receiver.recv() {
                    Ok(msg) => msg,
                    Err(_) => break,
                }
            };

            match msg {
                Message::RunTask(task) => {
                    {
                        let mut state = task.state.lock().unwrap();
                        state.is_polling = true;
                        state.is_queued = false;
                    }

                    let is_pending = {
                        let mut future_slot = task.future.lock().unwrap();
                        if let Some(mut future) = future_slot.take() {
                            let waker = Waker::from(task.clone());
                            let mut context = Context::from_waker(&waker);
                            let res = future.as_mut().poll(&mut context);
                            if res.is_pending() {
                                *future_slot = Some(future);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    };

                    {
                        let mut state = task.state.lock().unwrap();
                        state.is_polling = false;
                        if is_pending && state.woken_during_poll {
                            state.woken_during_poll = false;
                            state.is_queued = true;
                            let _ = task.task_sender.send(Message::RunTask(task.clone()));
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
    executor_threads: Vec<std::thread::JoinHandle<()>>,
}

impl Runtime {
    pub fn new(num_threads: usize) -> Self {
        let (executor, sender) = Executor::new();
        let sender_clone = sender.clone();
        let mut executor_threads = Vec::with_capacity(num_threads);

        for i in 0..num_threads {
            let executor = executor.clone();
            let sender = sender.clone();
            let thread = std::thread::Builder::new()
                .name(format!("executor_thread_{}", i))
                .spawn(move || {
                    executor.run(sender);
                })
                .unwrap();
            executor_threads.push(thread);
        }

        Runtime {
            task_sender: sender_clone,
            executor_threads,
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
        let num_threads = self.executor_threads.len();
        for _ in 0..num_threads {
            let _ = self.task_sender.send(Message::Shutdown);
        }
        for thread in self.executor_threads.drain(..) {
            let _ = thread.join();
        }
    }
}
