/*
一个rust基本函数的定义方法
创建变量的时候可以不告诉他变量类型是什么
但是创建函数必须告诉它函数参数类型
rust编程语言里广泛使用的是带有下划线分割字母的代码风格
*/
fn max(a: i32, b: i32) -> i32 {
    //rust语言里并没有像其他语言(例如，JavaScript,cpp等)里的三元运算符
    //不过你可以返回一个判断表达试,如下:
    //rust语言里的函数是依靠最后的表达式是否带有分号来确定返回值
    if a < b {
        b
    } else {
        a
    } //return语句可以忽略，
}
fn main() {
    let name = "你好";
    println!("Hello, world! {}", name);
    println!("{}", max(8, 5));
}
