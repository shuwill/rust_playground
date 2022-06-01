/*
trait：定义共享行为
trait被用来向rust编译器描述某些特定类型拥有的，且能够被其他类型共享的功能；
它可以以一种抽象的方式来定义共享行为；
可以用trait约束来讲泛型参数指定为实现某些特定行为的类型；
类似于其他语言的接口，但是也不尽相同
 */

/*
定义trait
只有当trait或者类型定义于我们的库中时，才能为该类型实现对应的trait
 */
use std::fmt::{Debug, Display, format};

pub fn trait_practice() {
    let tweet = Tweet {
        username: String::from("wswei"),
        content: String::from("tweet content"),
        replt: String::from("replt"),
        retweet: String::from("retweet"),
    };

    tweet.summary_author();
}

pub trait Summary {
    //默认实现
    fn summarize(&self) -> String {
        format!("readmore...")
    }

    fn summary_author(&self) {
        let summary = self.summarize();
        println!("{}", summary);
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub replt: String,
    pub retweet: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//使用trait作为参数
pub fn notify(item: impl Summary) {
    println!("Breadking news! {}", item.summarize());
}

//trait约束
pub fn notify_summary<T: Summary>(item: T) {
    println!("Breadking news! {}", item.summarize());
}

//通过+语法来指定多个约束
pub fn notify_summary_displat(item: impl Summary + Display) {}

//使用where从句来简化trait约束
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}

fn some_function_where<T, U>(t: T, U: U)
    where T: Display + Clone,
          U: Clone + Debug {
    println!("trait where")
}

//返回实现trait的类型
fn returns_summart() -> impl Summary {
    Tweet {
        username: String::from("wswei"),
        content: String::from("tweet content"),
        replt: String::from("replt"),
        retweet: String::from("retweet"),
    }
}