/*
引用计数（reference counting）智能指针，这种指针会通过记录所有者的数量
来使一份数据被多个所有者同时持有，并在没有任何所有者时自动清理数据；

引用和智能指针之间还有一个差别：
引用是只借用数据的指针；与之相反，大多数智能指针本身就拥有它们指向的数据；

通常使用结构体来实现智能指针，区别一般结构体的地方在于它们会实现Deref和Drop这两个trait；
Deref trait使得智能指针结构的实例拥有于引用一致的行为，使我们可以编写出能够同时用于应用和智能指针的代码；
Drop trait使自定义的智能指针离开作用域时运行的代码

Box<T>：可用于在堆上分配值；
Rc<T>：允许多重所有权的引用技术类型；
Ref<T>和RefMut<T>，可以通过RefCell<T>访问，是一种可以在运行时而不是编译时执行借用规则的类型
 */

/*
Box<T>的使用场景：
1、当你用于一个无法在编译期确定大小的类型，但又想要在一个要求固定尺寸的上下文环境中使用这个类型的值时；
2、当需要传递大量数据的所有权，但又不希望产生大量数据的复制行为时；
3、当希望拥有一个现实了指定trait的类型值，但又不关心具体的类型时
 */

use std::cell::RefCell;
use std::ops::Deref;

struct List<T> {
    size: u32,
    first: Node<T>,
    last: Node<T>,
}


struct Node<T> {
    item: T,
    next: Box<Node<T>>,
}

#[test]
fn deref_taint_test() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    hello("rust");

    let java = MyBox::new(String::from("java"));
    hello(&java);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(str: &str) {
    println!("hello {}", str);
}

#[test]
fn drop_trait_test() {
    let csp = CustomSmartPointer {
        data: String::from("hello"),
    };

    //使用std:mem:drop提前丢弃值
    drop(csp);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quata!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quata!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quata!");
        }
    }
}

struct MockMessenger{
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger{sent_messages: RefCell::new(vec![])}
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(String::from(msg))
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}