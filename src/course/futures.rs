use std::future::Future;
use std::net::TcpListener;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
use futures::channel::mpsc::UnboundedReceiver;
use futures::executor::block_on;
use futures::future::{BoxFuture, ok};
use futures::FutureExt;
use futures::task::{ArcWake, waker_ref};

async fn do_somethings() {
    println!("go go go");
}

async fn hello_world() {
    hello_cat().await;
    println!("hello world");
}

async fn hello_cat() {
    println!("hello cat");
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: "稻香".to_string(),
    }
}

async fn sing_song(song: &Song) {
    println!("给大家献上一首{}的歌{}", song.author, song.name);
}

async fn dance() {
    println!("还记得你说家是唯一的城堡....");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(&song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

#[test]
fn test_futures() {
    block_on(async_main());
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = Arc::clone(&shared_state);
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waler) = shared_state.waker.take() {
                waler.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}


struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&waker);
                if future.as_mut().poll(&mut context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[test]
fn test_timer_future() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn( async {
        println!("howdy");
        TimerFuture::new(Duration::new(2,0)).await;
        println!("done!");
    });

    drop(spawner);

    executor.run();
}