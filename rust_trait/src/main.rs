/*
Trait
-Trait告诉rust编译器:
--某种类型具有哪些并且可以与其他类型共享的功能
-Trait:以抽象的定义共享行为
-Trait bounds(约束):泛型类型参数指定为实现了特定行为的类型
-Trait与其他语言里的接口(interface)类似，但有些区别
*/

/*
定义一个Trait
-Trait的定义:把方法签名放在一起，来定义实现某种目的所必需的一组行为。
--关键字:trait
--只有方法签名，没有具体实现
--trait可以有多个方法:每个方法签名占一行，以;结尾
--实现该trait的类型必须提供具体的方法实现
例子:
*/
pub trait Summary {
    //NewsArticle
    //Twitter
    fn summarize(&self) -> String;
    fn summarize1(&self) -> String;
}

fn main() {
    /*
    实现trait的约束
    可以在某个类型上实现某个trait的前提条件时:
    -这个类型 或 这个trait是在本地crate里定义的
    无法为外部类型来实现外部的trait:
    -这个限制是程序属性的一部分(也就是一致性)
    -更具体地说是孤儿规则:之所以这样命名是因为父类型不存在。
    -此规则确保其他人的代码不能破坏您的代码，反之亦然。
    -如果没有这个规则，两个crate可以为同以类型实现同一个trait，rust就不知道应该使用哪个实现了
    */
    //实地调用lib.rs文件里面的summary这个trait方法
    use rust_trait::Summary;
    use rust_trait::Tweet;
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
