use rand::Rng;

pub fn variables() {
    //rust中的变量默认是不可变的
    let x = 5;
    println!("The value of x is {}.", x);
    //x = 6;
    //常量
    const MAX_POINTS: u32 = 100_000;

    /*
    隐藏（shadow）
    隐藏机制与mut的区别在于：由于重复使用let关键字会创建出新的变量，
    所以我们可以在复用变量名称的同时改变它的类型
     */
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}.", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces is {}", spaces);

    /*
    标量类型
    标量类型是单个值类型的统称，rust内建4种基础的标量类型：整数，浮点数，布尔值以及字符
     */

    //整数字面量
    let integer_literals = 98_222;
    let integer_literals = 0xff;
    let integer_literals = 0o77;
    let integer_literals = 0b1111_0000;
    let integer_literals = b'A';

    //浮点数类型
    let float = 2.0;
    let float: f32 = 2.0;
    let r = 1 / 2;
    println!("The value of r is {}", r);

    //布尔类型
    let t = true;
    let f: bool = false;

    //字符类型，char类型占据4个字节，是一个unicode标量值
    let c = 'a';
    let c = '😊';
    println!("The value of c is {}", c);

    /*
    复合类型：复合类型可以讲不用类型的值组合为一个类型；
    rust内置了两种基础的复合类型：元组（tuple），数组（array）
     */

    //元组类型
    //元组类型可以将不同类型的多个值组合进一个复合类型中，元组拥有一个固定的长度，无法在生命结束后增加或减少其中的元素数量
    let tup: (i32, f64, bool, char) = (500, 3.14, true, '💗');
    //由于一个元组被视作为一个单独的复合元素，变量tup被绑定到了整个元组上，为了从元组中获得单个的值，可以使用模式匹配来解构元组
    let (i, f, b, c) = tup;
    println!("The value of the tup's c is {}", c);
    //除了上面的解构，我们可以通过索引并使用点号（.）来访问元组中的值
    println!("The value of the tup's i is {}", tup.0);

    //数组
    //和其他语言一样，数组中的元素必须是相同类型的，并且数组的长度是固定的，一旦生命就无法更改
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    //let array = [3,3,3,3,3]
    let array = [3; 5];
    //和其他语言一样，数组通过索引访问数组中的元素
    println!("{}", array[0])
}

/*
函数
rust代码使用蛇形命名法来规法函数和变量名称的风格：只使用小写的字母进行命名，并以下划线分割单词
 */

//函数参数
//在英语技术文档中，参数变量和传入的具体参数值有自己对应的名称parameter和argument
//在函数签名中，必须显示地声明每个参数的类型
pub fn println_int(int: i32) {
    println!("The value of int is {}", int)
}

//函数体中的语句和表达式
//rust是一门基于表达式的语言，所以它将语句（statement）与表达式（expression）区别为两个不同的概念
//语句是指那些操作但不返回值的指令 let x = 6;
//表达式是指会进行计算并产生一个值作为结果的指令
pub fn statement_expression() {
    //语句没有返回值，所以不能将其赋值给另一个变量
    //let x = (let y =6);
    let x = 6;
    //表达式
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y)
}

//函数的返回值
//rust中不用为返回值命名，但需要在箭头符号（->）的后面声明它的类型，在rust中函数的返回值等同于函数体最后一个表达式的值
//也可以使用return关键字并指定一个值来提前从函数中返回
pub fn sum(a: i32, b: i32) -> i32 {
    //a + b; 如果加上分号，那么表达式就会变成语句导致编译错误
    a + b
}

//控制流
pub fn control_flow() {
    //if表达式
    let intput = 5;
    let result = if intput < 6 {
        intput + 1
    } else {
        intput - 1
    };
    println!("The value of the result is {}", result);

    //循环
    //rust提供了三种循环：loop，while，for
    let mut count = 0;
    loop {
        count += 1;
        if count > 100 {
            break;
        }
    }
    println!("The value of count is {}", count);

    while count != 200 {
        count += 1;
    }
    println!("The value of count is {}", count);

    let array = [rand::thread_rng().gen_range(1..100); 5];
    let mut index = 0;
    while index < array.len() {
        println!("The value of the index {} in array is {}.", index, array[index]);
        index += 1;
    }

    for element in array.iter() {
        println!("The value of the array is {}.", element);
    }
}
