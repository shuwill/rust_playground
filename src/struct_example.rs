pub fn area(with: i32, height: i32) -> i32 {
    with * height
}

//使用元组重构上面代码
pub fn area_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

//定义方法
//和C++, Java等语言不一样，
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }

    //关联函数
    /*
    除了方法，impl块还允许自定义不同接受self作为参数的函数；
    由于这类函数和结构体相互关联，被称为关联函数；
    关联函数不会作用于某个具体的结构体实例，类似于java中的类的静态方法
    关联函数常常被用作构造器来返回一个结构体的新实例
     */
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area_rectangle(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}

pub fn calculate_area() {
    let rect = Rectangle {
        width: 100,
        height: 50,
    };

    let area = area_rectangle(&rect);
    println!("The area of {:?} is {}", rect, area);
    println!("The area of {:?} is {}", rect, rect.area());

    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("The rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("{:?}", rect1);
    println!("square: {:?}", Rectangle::square(16));

}