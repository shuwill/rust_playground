use std::thread;
use std::time::Duration;

/*
闭包：
rust中的闭包是一种可以存入变量，或作为参数传递给其他函数的匿名函数
 */

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today, do {} pushups", simulated_expensive_calculation(intensity));
        println!("Next, do {} situps", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today, Remember to stay hydrated!")
        } else {
            println!("Today, run for {} minutes", simulated_expensive_calculation(intensity));
        }
    }
}

//使用闭包存储代码进行重构
fn generate_workout_closures(intensity: u32, random_number: u32) {
    let expensive_closures = |num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {:?} pushups", expensive_closures(intensity));
        println!("Next, do {:?} situps", expensive_closures(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today, Remember to stay hydrated!")
        } else {
            println!("Today, run for {:?} minutes", expensive_closures(intensity));
        }
    }
}

//使用泛型参数和Fn trait来存储闭包
struct Cache<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cache<T> {
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_cache(intensity: u32, random_number: u32) {
    let mut cache = Cache::new(|num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {:?} pushups", cache.value(intensity));
        println!("Next, do {:?} situps", cache.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today, Remember to stay hydrated!")
        } else {
            println!("Today, run for {:?} minutes", cache.value(intensity));
        }
    }
}

//使用闭包捕获上下文环境
//闭包可以捕获自己所在的环境并且访问自己被定义时的作用域中的变量
#[test]
fn closures_scope_test() {
    let x = 4;
    let equal_to_x = |z| z == x;

    assert!(equal_to_x(4));

    let mut s1 = String::from("1233");
    let mut append_something = |s| {
        s1.push_str(s);
    };
    append_something("12333");
}
/*
FnOnce: 获取变量的所有权
FnMut：从环境中可变地借用值并对它们进行修改
Fn：从环境中不可变地借用值
 */

#[test]
fn test_generate_workout_closures() {
    generate_workout(10, 7);
}

#[test]
fn test_generate_workout_cache() {
    generate_workout(10, 7);
}

/*
使用迭代器处理元素序列
迭代器模式允许你依次为序列中的每一个元素执行某些任务
rust中的迭代器是惰性的，意味着创建迭代器后，除非主动调用方法来消耗并使用迭代器，否则它们不会产生任何实际效果
 */

impl<T> Iterator for Cache<T>
    where T: Fn(u32) -> u32 {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: &[Shoe], shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn shoes_in_my_size_test() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(&shoes, 10);
    println!("{:?}", in_my_size);
    println!("{:?}", shoes);
}

//使用Iterator trait来创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_test() {
    let counter = Counter::new();
    for index in counter {
        println!("{}", index);
    }

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| { a * b })
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);

    let s = "hello rust";
    let  words = s.split(" ");
    let n = words.count();
    //println!("{:?}", words)
}