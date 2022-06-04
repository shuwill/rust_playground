use std::cell::RefCell;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

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
        let handle = thread::spawn(move ||{
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