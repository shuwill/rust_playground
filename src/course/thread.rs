use std::cell::RefCell;
use std::os::unix::raw::mode_t;
use std::sync::{Arc, Condvar, mpsc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use rand::Rng;

#[test]
fn thread_spawn_test() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawn thread!", i)
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    //使用join句柄等待所有线程结束
    handle.join().unwrap();
}

//在线程中使用move闭包
//move闭包经常被用来于thread::spawn函数使用，它允许你在某个线程中使用来自另一个线程的数据
#[test]
fn thread_move_test() {
    let datas = vec![12, 23, 12, 23];
    let handle = thread::spawn(move || {
        println!("{:?}", datas)
    });
    handle.join().unwrap();
}

//使用消息传递在线程间转移数据
#[test]
fn mpsc_test() {
    //mpsc:multiple producer single consumer(多个生产者，单个消费者)
    let (sender, receiver) = mpsc::channel();

    let num_cpus = num_cpus::get();
    println!("num of cpu is {}.", num_cpus);

    for _ in 0..num_cpus {
        let sender = mpsc::Sender::clone(&sender);
        thread::spawn(move || {
            let val = format!("hi: {:?}", thread::current().id());
            sender.send(val).unwrap();
        });
    }


    // recv会阻塞当前线程直到有值被传入通过
    for received in receiver {
        println!("{}", received);
    }
}

/*
互斥体一次只允许一个线程访问数据
1、必须在使用数据前尝试获取锁；
2、必须在使用完互斥体守护的数据后释放锁，这样其他线程才能继续完成获取锁的操作
 */
#[test]
fn mutex_test() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for _ in 0..num_cpus::get() {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", counter);
}

#[test]
fn rw_lock_test() {
    let lock = Arc::new(RwLock::new(0));
    let mut threads = Vec::new();
    for _ in 0..num_cpus::get() {
        let lock = Arc::clone(&lock);
        let thread = thread::spawn(move || {
            let timeout = rand::thread_rng().gen_range(500..1000);
            thread::sleep(Duration::from_millis(timeout));
            let mut num = lock.write().unwrap();
            *num += 1;
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
    println!("{}", lock.read().unwrap())
}

#[test]
fn thread_local_test() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f|{
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let thread = thread::spawn(move || {
        FOO.with(|f|{
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    thread.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    })
}

#[test]
fn condvar_test() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = Arc::clone(&pair);

    thread::spawn(move || {
        let &(ref lock, ref cvar) = & *pair1;
        let mut started = lock.lock().unwrap();
        println!("change start");
        *started = true;
        thread::sleep(Duration::from_secs(4));
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = & *pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("change start");

}