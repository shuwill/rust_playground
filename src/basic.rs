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
 */
