pub fn generic_type() {
    let vec = vec![12, 23, 34, 13];
    match largest(&vec) {
        Ok(value) => println!("The largest value of vec is {}.", value),
        Err(err) => println!("{}", err),
    }

    let vec = vec!['a', 'b'];
    match largest(&vec) {
        Ok(value) => println!("The largest value of vec is {}.", value),
        Err(err) => println!("{}", err),
    }

    let int = Point { x: 123, y: 321 };
    let float = Point { x: 12.3, y: 32.1 };

    let status_code = Status::COMPLETE(0);
    let status_string = Status::COMPLETE("完成");
    println!("{:?}-{:?}", status_code, status_string);

    let p1 = Point {
        x: 1.0,
        y: 1.0,
    };

    let p2 = Point {
        x: 10.0,
        y: 10.0,
    };

    let distance = p1.distance(&p2);
    println!("{}", distance);
}

fn largest_i32(vec: &Vec<i32>) -> Result<&i32, &str> {
    if vec.len() > 0 {
        let mut largest = &vec[0];
        for element in vec {
            if element > largest {
                largest = element;
            }
        }
        return Ok(largest);
    }
    Err("vector is empty")
}

/*
泛型数据类型
我们可以在声明函数签名或结构体等元素时使用泛型
 */

//当使用泛型来定义一个函数时，我们需要将泛型放置在函数签名中，通常用于指定参数和返回值类型的地方
fn largest<T: Ord>(vec: &Vec<T>) -> Result<&T, &str> {
    if vec.len() > 0 {
        let mut largest = &vec[0];
        for element in vec {
            if element > largest {
                largest = element;
            }
        }
        return Ok(largest);
    }
    Err("vector is empty")
}

//在结构体中定义
struct Point<T> {
    x: T,
    y: T,
}

//在枚举中定义
#[derive(Debug)]
enum Status<T> {
    COMPLETE(T),
    PROCESSING(T),
}

//在方法中定义
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance(&self, other: &Self) -> f64 {
        let x2 = (self.x - other.x).abs().powf(2.0);
        let y2 = (self.y - other.y).abs().powf(2.0);
        (x2 + y2).sqrt()
    }
}