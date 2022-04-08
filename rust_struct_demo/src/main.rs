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

//结构体方法
/*
方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。
如果你学习过一些面向对象的语言，那你一定很清楚函数一般放在类定义里并在函数中用 this 表示所操作的实例。
Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。
结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
贴士：结构体 impl 块可以写几次，效果相当于它们内容的拼接！
*/
impl Rectangle {
    fn area(&self) -> u32 {
        //计算长方形的面积
        self.width * self.length
    }
    fn wider(&self, rect: &Rectangle) -> bool {
        //计算某个长方形是否比另一个长方形更宽。
        self.width > rect.width
    }
}

/*
结构体关联函数
之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。
这种函数 "不依赖实例" ，但是使用它需要声明是在哪个 impl 块中的。
一直使用的 String::from 函数就是一个"关联函数"。
*/

impl Rectangle {
    fn create(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 500,
        length: 400,
    };

    let rect2 = Rectangle {
        width: 700,
        length: 800,
    };

    //rect3是利用结构体的关联函数creact创建的
    let rect3 = Rectangle::create(30, 50);

    //利用了16行功能实现的直接显示此实例的内容
    //:?是Debug模式下的格式化字符串
    //:#?加了个井号可以pretty输出
    println!("rect1 -> {:#?}", rect1);
    println!("rect2 -> {:#?}", rect2);
    println!("rect3 -> {:#?}", rect3);

    println!("rect1 area is {}", rect1.area());
    println!("rect2 area is {}", rect2.area());
    println!("rect1 > rect2 ? : {}", rect1.wider(&rect2));
}
