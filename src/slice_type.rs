//切片
/*
除了引用，rust还有另一种不持有所有权的数据类型：切片
切片允许我们引用集合中某一段连续的元素序列，而不是整个集合
 */
pub fn slice_type() {
    let mut str = String::from("hello world");
    let index = first_word(&str);
    println!("{}", index);

    let hello = &str[0..5];
    let word = &str[6..11];
    println!("{}, {}", hello, word);
    /*
    字符串切片的边界必须位于有效的UTF-8字符边界内，尝试从一个多字节字符的中间位置创建字符串切片会导致运行时错误
     */
    let first_space = first_word(&str);
    //str.clear(); 借用规则，当我们拥有某个变量的不可变引用时，我们就无法读取该变量的可变引用
    println!("{}", first_space);

    println!("{}", first_word_better("rust hello"));
    println!("{}", first_word_better(&str[3..]))

}

fn first_word_old(str: &String) -> usize {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    str.len()
}

fn first_word(str: &String) -> &str{
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &str[..i];
        }
    }
    &str[..]
}

fn first_word_better(str: &str) -> &str{
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &str[..i];
        }
    }
    &str[..]
}