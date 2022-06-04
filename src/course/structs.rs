//使用结构体来组织相关联的数据

//定义并实例化结构体
/*
和元组一样，结构体中的数据可以拥有不用的类型；不同的是结构体需要给每个数据赋予名字
以便清楚地表明它们的意义
 */
pub struct User {
    pub username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}

pub fn init_user() -> User {
    User {
        email: String::from("someone@example.com"),
        username: String::from("someone_username"),
        sign_in_count: 10,
        active: true,
    }
}

pub fn user_demo() {
    let user = init_user();
    println!("{}", user.username);

    //一旦实例可变，那么实例中的所有字段都是可变的，rust不允许我们单独声明某一部分字段的可变性
    let mut user_1 = user;
    user_1.username = String::from("1233");

    println!("{}", user_1.username);

    let new_user = build_with_other_user(
        String::from("123@qq.com"),
        String::from("123"),
        &user_1
    );
    println!("{}", new_user.username);

}

//在变量名与字段名相同时使用简化版的字段初始化方法
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 100,
        active: true,
    }
}

//使用结构体更新语法根据其他实例创建新实例
fn build_with_other_user(email: String, username:String, user: &User) -> User {
    User{
        username,
        email,
        sign_in_count: user.sign_in_count,
        active: user.active,
    }
}

//使用不需要对字段命名的元组结构体来创建不同的类型
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn tuple_struct() {
    let black = Color(0,0,0);
    let origin =Point(0,0, 0);
    //black和origin是不同的类型，因为它们分别是不同的元组结构体的实例
    //元组结构体实例的行为就像元组一样：可以使用模式匹配将它们解构为单独的部分，也可以用过.以及索引来访问特定的字段
    let Color(r, g, b) = black;
    println!("color red is {}", r)

}

//没有任何字段的空结构体
struct Unit;

//结构体数据的所有权
struct Student{
    //name: &str,
    age: u8,
}