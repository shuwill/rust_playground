/*
使用生命周期保证引用的有效性
rust中的每个引用都有自己的lifetime，它对应者引用保持有效性的作用域
大多数时候，生命周期都是隐式且可以被推导出来
 */

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(part: &str) -> ImportantExcerpt {
        ImportantExcerpt { part: part }
    }
}

/*
生命周期标注语法
生命周期的标注并不会改变任何引用的生命周期长度
使用了生命周期的函数可以接受任何带有生命周期的引用
 */

pub fn lifetimes() {
    let s1 = "hello";
    let s2 = "rust";
    println!("{}", longest(s1, s2));

    let ie1 = ImportantExcerpt::new("12344");
    println!("{}", ie1.part)
}

/*
rust编译器生命周期的三中计算规则
1、每一个引用参数都会拥有自己的生命周期参数；
2、当只存在一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数
3、当拥有多个输入生命周期参数，而其中一个是&self或&mut self时，self的生命周期会被赋予给所有的输出生命周期参数
 */