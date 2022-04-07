/*
例子需求:
计算长方形的面积
*/
//优化前:可维护性差,参数间没有关联
// fn main() {
//     let width = 500;
//     let length = 400;
//     println!("{}",area(width, length));
// }
// fn area(width: u32, length: u32) -> u32 {
//     width * length
// }

//利用struct后
#[derive(Debug)] //我是一个注解,表示我下面这个自定义的类型派生于Debug来做一些功能
                 //比如直接使用println!宏来打印此Rectangle的结构体实例
struct Rectangle {
    width: u32,
    length: u32,
}
fn main() {
    let rect = Rectangle {
        width: 500,
        length: 400,
    };
    //利用了16行功能实现的直接显示此实例的内容
    //:?是Debug模式下的格式化字符串
    //:#?加了个井号可以pretty输出
    println!("{:#?}", rect);
    println!("{}", area(&rect));
}
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
