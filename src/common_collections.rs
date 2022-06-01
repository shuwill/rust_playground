use std::collections::HashMap;
use std::mem::uninitialized;

//动态数组 vector
//字符串 string
//哈希映射 hash map

/*
动态数组允许你在单个数据结构中存储多个相同类型的值；
这些值会彼此相邻的排布在内存中
 */
pub fn vector() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(4);
    v1.push(5);
    let mut v3 = v1;

    v3.sort();

    //读取动态数组中的元素
    let third: &i32 = &v3[2];
    println!("The third element of v3 is {}", third);
    v3.push(6);

    let third = v3[2];
    v3.push(6);
    println!("The third element of v3 is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element of v3 is {}", third),
        None => (),
    };
    v3.push(6);

    let mut strings: Vec<&str> = Vec::new();
    strings.push("hello");
    strings.push("你好");
    strings.push("😊");

    let s3 = strings[2];
    strings.push("1233");
    println!("The third element of strings is {}", s3);

    match strings.get(2) {
        Some(str) => println!("The third element of strings is {}", str),
        None => (),
    };

    for i in &mut v3 {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.65),
        SpreadsheetCell::Text(String::from("1233")),
    ];

    for cell in &row {
        println!("{:?}", cell)
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/*
使用字符串存储utf-8编码的文本
rust在语言核心部分只有一种字符串类型，就是字符串切片str，通常以借用的形式（&str）出现;
字符串字面量的数据被存储在程序的二进制文件中，它们本身也是字符串切片的一种
 */
pub fn string() {
    let mut s = String::new();

    let data = "initial_contents";
    let mut s = data.to_string();

    let char = s.get(0..2);
    let sr = &s;
    println!("{}", sr);
    s.push_str("1233");

    let s1 = String::from("hello");
    let s2 = String::from("word");
    let s3 = s1 + &s2;
    /*
    这里的+运算符会调用一个add方法，
    编译器可以自动将&String类型的参数强制转化为&Str类型；
    当调用add方法时，rust使用一种被称为解引用强制转化的技术，将&s2转化为&s2[..]
     */
    println!("{}", s3);

    /*
    如果需要拼接多个字符串，使用+运算符显得十分笨拙
    所以对于一些复杂的字符串拼接，我们使用format!宏，它不会夺取任何参数的所有权
     */
    let s1 = String::from("hello");
    let s2 = String::from("word");
    let s3 = format!("{}-{}", s1, s2);
    println!("{}-{}-{}", s1, s2, s3);

    //rust不允许我们通过索引来获取String中的字符

    let hello = "你好";
    for b in hello.as_bytes() {
        println!("{}", b);
    };

    for c in hello.chars() {
        println!("{}", c);
    }
}

pub fn hashmap() {
    let mut sources: HashMap<String, i32> = HashMap::new();
    sources.insert(String::from("Blue"), 10);
    sources.insert(String::from("Yellow"), 50);

    //和其他语言类似，要求所有的键必须拥有相同的类型，所有的值必须拥有相同的类型

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_sources = vec![10, 50];
    let mut sources: HashMap<_, _> = teams.iter()
        .zip(initial_sources.iter()).collect();
    sources.insert(&String::from("Red"), &100);

    /*
    哈希映射的所有权
    对于实现了Copy trait的类型，它们的值会被简单地复制到哈希映射中；
    但是对于类似String，所有权会被转移到哈希映射中
     */
    let k1 = String::from("Blue");
    let k2 = String::from("Yellow");
    let mut sources: HashMap<String, i32> = HashMap::new();
    sources.insert(k1, 10);
    sources.insert(k2, 50);
    //println!("{}, {}", k1, k2); k1和k2的所有权会被转移到sources中

    //访问哈希映射中的值
    let score = sources.get("Blue");
    match score {
        Some(int) => { println!("{}", int) }
        None => (),
    };

    for (key, value) in &sources {
        println!("{}, {}", key, value);
    }

    /*
    更新哈希映射
     */
    //覆盖旧值
    sources.insert(String::from("Blue"), 90);
    println!("{:?}", sources);

    //如果key不存在则插入
    sources.entry(String::from("Blue")).or_insert(190);
    sources.entry(String::from("Red")).or_insert(190);
    println!("{:?}", sources);

    //基于旧值更新值
    let text = "hello word hello world hello";
    let mut text_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_count.entry(word).or_insert(0);
        *count += 1;
    };
    println!("{:?}", text_count);

    let mut goods_tags = HashMap::new();
    let goods_tag = GoodsTag {
        id: 12345,
        tag: String::from("fruit"),
    };

    goods_tags.insert("apple", &goods_tag);
    goods_tags.insert("banana", &goods_tag);

    for (key, value) in goods_tags {
        println!("{}:{:?}", key, value);
    }
}

#[derive(Debug)]
struct GoodsTag {
    id: i32,
    tag: String,
}