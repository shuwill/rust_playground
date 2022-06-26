//引用与借用
pub fn references_borrowing() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of s is {}", len);
    println!("{}", s);
    let s1 = s;
    println!("{}", s1);

    let (x, y) = (String::from("12"), String::from("13"));
    //swap(x, y);
    let (x, y) = swap(x, y);
    println!("The value of x is {}, value of y is {}", x, y);

    let mut str = String::from("rust");
    append(&mut str);
    println!("{}", str);

    /*
    可变引用在使用上有一个很大的限制：对于特定作用域中的特定参数来说，一次只能声明一个可变引用
     */
    let mut str = String::from("hello");
    //let s1 = &mut str;
    //let s2 = &mut str;
    //println!("{}, {}", s1, s2);

    /*
    不能在拥有不可变引用的同时创建可变引用
     */
    //let s1 = &str;
    //let s2 = &str;

    //let s3 = &mut str;
    //println!("{}, {}, {}", s1, s2, s3);

    let s1 = &str;
    let s2 = &str;
    println!("{}, {}", s1, s2);

    let s3 = &mut str;
    println!("{}", s3);

    //dangle();
}

/*
&代表的就是引用语义，它们允许你在不获取所有权的前提下使用值;
与使用&进行引用相反的操作被称为解引用（dereferencing）,它使用*作为运算符
通过引用传递参数给函数的方法也被称为借用（borrowing）
与变量类似，引用默认是不可变的
 */
fn calculate_length(str: &String) -> usize {
    str.len()
}

//可变引用
fn append(str: &mut String) {
    //str.push_str("1233");
    str.push_str("1233");
}

//悬垂引用
/*
fn dangle() -> &String{
    let s = String::from("hello");
    &s
}
*/

fn swap(mut x: String, mut y: String) -> (String, String) {
    let tmp = y;
    y = x;
    x = tmp;
    (x, y)
}

/*
引用规则：
1、在任何一段给定的时间里，你要么只能拥有一个不可变引用，要么只能拥有任意数量的不可变引用
2、引用总是有效的
 */

fn strtok<'a>(str: &'a mut &str, delimiter: char) -> &'a str {
    if let Some(i) = str.find(delimiter) {
        let prefix = &str[..i];
        *str = &str[(i + delimiter.len_utf8())..];
        return prefix;
    } else {

        ""
    }
}

#[test]
fn strtok_test() {
    let mut str = "hello world";

    let hello = strtok(&mut str, ' ');
    assert_eq!(hello, "hello");

    assert_eq!("world", str);
}