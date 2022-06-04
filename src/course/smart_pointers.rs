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

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

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

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { sent_messages: RefCell::new(vec![]) }
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

struct TreeNode<T> {
    item: T,
    parent: Option<Box<TreeNode<T>>>,
    childs: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    fn new(item: T) -> TreeNode<T> {
        TreeNode {
            item,
            parent: None,
            childs: vec![],
        }
    }
}

#[derive(Debug)]
struct Owner {
    name: String,
}

struct GadgetLifeTime<'a> {
    id: i32,
    owner: &'a Owner,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

#[test]
fn rc_test() {
    let s1 = String::from("hello");
    //let b1 = Box::new(s1);
    //let b2 = Box::new(s1);

    let mut r1 = Rc::new(s1);
    println!("r1 count: {}", Rc::strong_count(&r1));
    {
        let r2 = Rc::clone(&r1);
        println!("r2 count: {}", Rc::strong_count(&r2));
    }
    println!("r1 count: {}", Rc::strong_count(&r1));
    //r1.push_str("1233");

    let owner = Owner { name: String::from("Tom") };
    let g1 = GadgetLifeTime { id: 1, owner: &owner };
    let g2 = GadgetLifeTime { id: 2, owner: &owner };

    drop(owner);
    //println!("the owner of g1 is {:?}.", g1.owner);
    //println!("the owner of g2 is {:?}.", g2.owner);

    //let's begin use Rc to change this
    let owner_rc = Rc::new(Owner { name: String::from("Tom") });
    let g1 = Gadget { id: 1, owner: Rc::clone(&owner_rc) };
    let g2 = Gadget { id: 2, owner: Rc::clone(&owner_rc) };

    drop(owner_rc);
    println!("the owner of g1 is {:?}.", g1.owner);
    println!("the owner of g2 is {:?}.", g2.owner);

    //多线程无力的Rc
    let s = Rc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let rc = Rc::clone(&s);
        /*
        thread::spawn(move ||{
           println!("{}", rc);
        });
        */
    }
    //Rc需要管理引用计数，但是它并没有实现任何并发原语，因此无法实现原子化操作，最终会使计数错误

    //Arc为了解决Rc在多线程中无法正确进行引用计数应用而生
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let arc = Arc::clone(&s);
        thread::spawn(move || {
            println!("{:?}: {}", thread::current().id(), arc);
        });
    }

    /*
    rust中，所有权机制保证一个数据只有一个所有者，但是如果在图数据结构中，多线程等场景中共享数据，这种机制会成为极大的阻碍；
    rust提供了智能指针Ac和Arc，使用它们就能实现多个所有者共享一个数据的功能；
    Rc和Arc的区别在于：后者是原子化实现的引用，是线程安全的；
    但是两者都是只读的，如果想要实现内部数据可修改，必须配合内部可变性ReCell或者互斥锁Mutex来一起使用
     */
}

#[test]
fn cell_test() {
    //cell和RefCell在功能上没有区别，区别在于Cell<T>适用于T实现了Copy的情况
    let c = Cell::new("hello");
    let s = c.get();
    c.set("rust");
    println!("{} {}", s, c.get());

    //RefCell实现了编译器可变，不可变引用共存
    let ref_cell = RefCell::new(String::from("hello RefCell"));
    //let s1 = ref_cell.borrow();
    //let s2 = ref_cell.borrow_mut();
    //println!("{}, {}", s1, s2);
    /*
    上面的代码虽然可以编译通过，但是在运行还是会panic的，因为它违背了借用规则
     */
}

//定义在外部库中的trait
pub trait Producer {
    fn send(&self, msg: String);
}

//我们自定义的Producer
struct MyProducer {
    messages: Vec<String>,
}

struct MyReCellProducer {
    messages: RefCell<Vec<String>>,
}

//实现Producer
impl Producer for MyProducer {
    fn send(&self, msg: String) {
        //self.messages.insert(msg);
    }
}

//ReCell✨登场
impl Producer for MyReCellProducer {
    fn send(&self, msg: String) {
        self.messages.borrow_mut().push(msg);
    }
}

#[test]
fn rc_ref_cell_test() {
    let s = Rc::new(RefCell::new("hello".to_string()));
    let s1 = Rc::clone(&s);
    let s2 = Rc::clone(&s);

    s.borrow_mut().push_str("12333");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    };
    nums.truncate(i);
}

struct SelfRef<'a>{
    value: String,
    pointer_to_value: &'a str,
}

#[test]
fn self_ref_test() {
    let s = String::from("hello");
}