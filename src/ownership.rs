/*
所有权
rust使用特定规则的所有权系统来管理内存，这套规则允许编译器在编译过程中执行检查工作，而不会产生任何的运行时开销
栈与堆
所有存储在栈中的数据必须拥有已知且固定的大小；对于在编译期间无法确定大小的数据，就只能将它们存储在堆中。
1、rust中的每一个值都有一个对应的变量作为它的所有者；
2、在同一时间内，值有且仅有一个所有者
3、当所有者离开自己的作用域时，它持有的值就会被释放掉
 */

pub fn variables_scope() {
    {
        let s = "hello";
    }
    // println!("{}", s);
    /*
    s在进入作用域后变得有效；
    它会保持自己的有效性直到自己离开作用域为止。
     */

    //rust中的标量类型和复合类型（基本类型）都会将数据存储在栈上，并在离开自己的作用域时将数据弹出栈空间

    //example string
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    /*
    对于字符串字面量而言，由于在编译是就知道其内容；
    对于String类型而言，为了支持一个可变的，可增长的文本类型，需要在堆上分配一块在编译是未知大小的内存来存放数据，这意味着：
    1、使用的内存是有操作系统在运行是动态分配出来的；
    2、在使用完String时，需要通过某种方式将内存归还操作系统
    上面代码通过String::from来请求自己需要的内存空间完成第一步；
    对于第二步来讲不同的编程语言有自己的不同的做法，某些拥有GC机制的语言（例如Java）,GC会替代程序员负责标记并记录那些不再使用的
    内存；对于没有GC的语言（例如C，C++）需要程序员显式地释放内存；
    rust提供了另一套解决方案：
    内存自动地在拥有它的变量离开作用域后进行释放
     */
    {
        let mut s = String::from("rust");
        {
            s.push_str(", java");
        }
        s.push_str(", kotlin");
        println!("{}", s)
    } //作用域到这里结束，变量s失效

    {
        let i1 = 5;
        let i2 = i1;
        /*
        对于在栈上分配的类型来讲，i2会创建一个i1值的拷贝，并且绑定到i2上，深拷贝
         */
    }


    {
        let s1 = String::from("hello");
        let s2 = s1;
        /*
        对于在堆上分配的类型来讲，rust不会复制值时深度复制堆上的数据，浅拷贝，move
        s1和s2同时指向堆上的一块内存区域，当s1和s2同时离开作用域时，会尝试重复释放相同的内存，进而可能引发错误;
        为了确保内存安全，同时也避免复制分配的内存，rust在次场景中会简单地将s1废弃掉，不再视其为一个有效的变量;
         */
        // println!("{}", s1); 试图在s2创建完毕后使用s1会导致编译错误

        //如果确实需要深度拷贝String堆上的数据，而不仅仅是栈上数据时，可以使用clone
        let s3 = s2.clone();
        println!("{}, {}", s2, s3);

        /*
        栈上的数据复制
        由于类似于整型的类型可以在编译是确定自己的大小，并且能够将自己的数据完整地存储在栈中，完全不需要在类似场景中考虑上面的问题
         */
        let tup1: (i32, f64, bool) = (123, 12.0, true);
        let tup2 = tup1;
        println!("{}", tup1.1);

        let array1 = [3; 5];
        let array2 = array1;
        for element in array1 {
            println!("{}", element)
        }
        /*
        rust提供了一个名为Copy的trait，用于整数这类完全存储在栈上的数据类型；
        一旦某种类型拥有了Copy这种trait，那么它的变量就可以在赋值给其他变量之后报错可用性；
        如果一种类型本身或者这种类型的任意成员实现了Drop这种trait，那么rust就不允许其实现Copy这种trait
         */
    }

    ownership_function();
    return_values_scope();
}

//所有权和函数
pub fn ownership_function() {
    /*
    将值传递给函数在语义上类似于对变量进行赋值，会触发移动或者复制
     */
    let s = String::from("hello");
    takes_ownership(s);
    //let s1 = s;

    let i1 = 123;
    makes_copy(i1);
    let i2 = i1;
}

fn takes_ownership(str: String) {
    println!("{}", str)
}

fn makes_copy(int: i32) {
    println!("{}", int);
}

/*
函数在返回值的过程中也会发生所有权的转移
 */
fn return_values_scope() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_ownership(s2);
    //println!("{}", s2);

    let string_len = calculate_length(s3);
    //println!("The length of {} is {}", s3, string_len.1);
    println!("The length of {} is {}", string_len.0, string_len.1);
}

fn gives_ownership() -> String {
    let str = String::from("hello");
    str
}

fn takes_and_gives_ownership(str: String) -> String {
    str
}
/*
变量所有权的转移总是遵循相同的模式，将一个值赋值给另一个变量时就会转移所有权，当一个持有堆数据的变量离开作用域时，
它的数据就会被drop清理回收，除非这些数据的所有权移动到另一个变量上
 */
fn calculate_length(str: String) -> (String, usize) {
    let len = str.len();
    (str, len)
}