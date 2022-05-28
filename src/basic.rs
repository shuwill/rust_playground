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
    let r = 1 / 2 ;
    println!("The value of r is {}", r);

    //字符类型
    let c = 'a';
    let c ='😊';
    println!("The value of c is {}", c);
}