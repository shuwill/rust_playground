/*
rust中将错误分成两类：可恢复错误与不可恢复错误
其他语言通过异常之类的机制统一处理它们，rust中没有异常机制；
但是提供了可用于恢复错误的类型Result<T,E>，以及在程序中出现不可恢复错误时终止运行的panic!宏
 */
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::net::IpAddr;

pub fn error_handling() {
    //panic!("this is a panic error!!")

    //可恢复错误与Result
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
           let metadata = file.metadata();
            match metadata {
                Ok(metadata) => {
                    println!("The file length is {}.", metadata.len())
                },
                Err(error) => {
                    println!("get the file metadata error: {}", error);
                }
            }
        },
        Err(error) => {
            //匹配错误类型
            match error.kind() {
                ErrorKind::NotFound => {
                    let file = File::create("hello.txt");
                    match file {
                        Ok(file) => println!("file create successful"),
                        Err(error) => println!("file create happen error: {}", error),
                    }
                },
                _ => {
                    println!("The file open error: {}", error);
                }
            }
        }
    }

    //闭包写法
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //失败时触发 panic的快捷方式：unwrap和expect
    let f = File::open("hello.txt").unwrap();
    let mut f = File::open("hello.txt").expect("File open hello.txt error");
    let mut f = OpenOptions::new().write(true).open("hello.txt").expect("");
    f.write("hello rust".as_bytes());

    let username = read_username_from_file();
    match username {
        Ok(name) => println!("{}", name),
        Err(e) => println!("{}", e),
    };

    let username = read_username_from_file_quikly();
    match username {
        Ok(name) => println!("{}", name),
        Err(e) => println!("{}", e),
    };

    let ip_addr = parse_ip_addr("127.0.0.1");
    println!("{}", ip_addr.is_ipv4());
}

//传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    return match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
}

//传播错误的快捷方式：?运算符
/*
?运算符只能被用于返回Result的函数
 */
fn read_username_from_file_quikly() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn parse_ip_addr(ip_addr: &str) -> IpAddr {
    ip_addr.parse().expect("illegal ip addr")
}