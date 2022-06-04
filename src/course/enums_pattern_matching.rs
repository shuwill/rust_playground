//枚举与模式匹配

use crate::course::enums_pattern_matching::Message::Move;

pub fn enums() {
    //枚举值
    let v4 = IpAddrKid::V4;
    let v6 = IpAddrKid::V6;

    route(&v4);
    route(&v6);

    println!("{:?}-{:?}", v4, v6);

    let home = IpAddr {
        ip_addr_kid: IpAddrKid::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        ip_addr_kid: IpAddrKid::V4,
        address: String::from("::1"),
    };

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("{:?}", home);

    let message_move = Message::Move {
        x: 100,
        y: 200,
    };
    message_move.call();

    //Option枚举及其在空值处理
    let some_number = Some(5);
    let some_string = Some("a string");

    //如果使用的变体是None，需要明确告知rust中Option<T>的具体类型
    let absent_number: Option<i32> = None;

    //Option<T>和T是不同的类型，所以编译器不会允许我们像使用普通值一样直接使用Option<T>的值
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    //let sum = x + y;
}

//定义枚举
#[derive(Debug)]
enum IpAddrKid {
    V4,
    V6,
}

fn route(ip_addr_kid: &IpAddrKid) {
    println!("{:?}", ip_addr_kid)
}

struct IpAddr {
    ip_addr_kid: IpAddrKid,
    address: String,
}

//每个枚举的变体可以拥有不同类型和数量的关联数据
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    //匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move{x,y} => {
                println!("move to [{}, {}]", x,y);
            }
            Message::Write(content) => {
                println!("write {}",content);
            }
            _ => (),
        }
    }
}

//控制流运算符match
/*
match：允许讲一个值与一系列的模式相比较，并根据匹配的模式执行相应的代码；
模式可由字面量，变量名，通配符和许多其他东西组成
匹配必须穷举所有的可能
_通配符
 */
pub fn pattern_match() {
    let peeny = Coin::Peeny;
    let coin_value = value_in_coin(&peeny);
    println!("The value of {:?} is {}", peeny, coin_value);

    let quarter = Coin::Quarter(UsState::Alabama);
    let quarter_value = value_in_coin(&quarter);
    println!("The value of {:?} is {}", quarter, quarter_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_number = 12;
    match some_number {
        1 => println!("one"),
        2 => println!("two"),
        _ => {
            println!("other number");
        }
    };

    //简单控制流if let
    let some_number = Some(3);
    match some_number {
        Some(3) => println!("three"),
        Some(4) => println!("four"),
        None => println!("None"),
        _ => {}
    }

    if let Some(3) = some_number{
        println!("three")
    };

    let count = count_in_coin(&peeny);
    println!("{}", count);

    let mut s = String::from("test");
    append_one(Some(&mut s));
    println!("{}", s);
}

#[derive(Debug)]
enum Coin {
    Peeny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_coin(coin: &Coin) -> u8 {
    match coin {
        Coin::Peeny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn count_in_coin(coin: &Coin) -> i32 {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("{:?}", state)
    } else {
        count += 1;
    }
    count
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn append_one(s: Option<&mut String>) -> Option<&String> {
    match s {
        None => None,
        Some(s) => {
            s.push_str("13233");
            Some(s)
        },
    }
}