//基本的输入输出
use std::io; //导入标准库
fn main() {
    println!("请输入您的姓名:");
    //创建一个用于接收字符串的变量,必须带有可修改关键字(mut)
    let mut name = String::new();
    io::stdin() //使用导入的标准库中的stdin函数
        .read_line(&mut name) //接收用户输入的内容存储的地方
        .expect("你输入的数据有点问题啊哥们!"); //出现意外问题的提示
    println!("您的姓名是{}", name);
}
