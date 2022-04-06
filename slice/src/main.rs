//rust切片
//方便对字符串进行操作/数组也可以进行切片操作
//注意事项:
//1、字符串切片的范围索引必须发生在有效的UTF-8字符边界内。
//2、如果尝试从一个多字节的字符中创建字符串切片,程序会报错并退出。
fn main() {
    //let hello=&s[0..5];   可简写成:let hello=&s[..5];  取字符串中的0-5之间包括0本身和5之前间的值
    //let world=&s[6..11];   可简写成:let hello=&s[6..]; 取字符串中的6-11之间包括6本身和11之前间的值
    //let all=&s[0..s.len()];  可简写成let all=&s[..];  取字符串从头到尾的全部值

    let my_string = String::from("hello world");
    let word_index = find_string(&my_string[..]); //因为接受的是一个字符串字面量引用，所以要利用切片来获取

    let my_string_literal = "hello world";
    let word_index_literal = find_string(my_string_literal); //因为使用的就是字符串字面量，所以可以直接传本身
    println!("string : {}", word_index);
    println!("str : {}", word_index_literal);
}
//实现读取一段字符串的第一个单词的函数
//str是字符串切片的类型
fn find_string(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
