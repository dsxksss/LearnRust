/*
在类型上实现trait
-与为类型实现方法类似。
-不同之处:
--impl Xxxx for Tweet{...}
--在impl的块里，需要对Trait里的方法签名进行具体的实现
例子:
*/

use core::fmt::Debug;
use std::fmt::Display;
pub trait Summary {
    // fn summarize(&self) -> String;
    //trait的默认实现:
    fn summarize(&self) -> String {
        String::from("(Read more ...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    //如果不定义NewsArticle的SummaryTrait实现，则会使用此Trait的默认实现方法
    //不定义的话可以是空的
    // fn summarize(&self) -> String {
    //     format!("{},by{} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//带参数的Tarit语法各实现
//impl Trait语法
pub fn notify1(item1: impl Summary + Display, _item2: impl Clone + Debug) -> String {
    format!("Breaking news! {}", item1.summarize())
}

//trait bound(泛型约束)语法
pub fn notify2<T: Summary + Display, U: Clone + Debug>(a: T, _b: U) -> String {
    format!("Breaking news! {}", a.summarize())
}

//where子句语法
pub fn notify3<T, U>(a: T, _b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}

/*
Trait作为参数
impl Trait语法:适用于简单情况
Trait bound语法:可用于复杂情况
-impl Trait语法是trait bound的语法糖
使用+指定多个Trait bound
Trait bound使用where子句
-在方法签名后指定where子句
*/

/*
实现Trait作为返回类型
例子:
*/

//impl Trait语法
//它的返回类型必须实现了Summary这个Trait
pub fn notify4(_s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from(
            "The Pittsburg Penguins ones again are the best hockey team in zhe NHL.",
        ),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA"),
    }
} //注意:impl Trait只能返回确定的同一种类型，返回可能不同类型的代码会报错

//Trait Bound语法:
//例子:使用Trait Bound修复在泛型笔记里的largest函数
//具体实现转倒泛型那篇笔记里

/*
使用Trait Bound有条件的实现方法
-在使用泛型类型的参数的impl块上使用Trait bound，
我们可以有条件的为实现了特定Trait的类型来实现方法
例子:
*/
struct Pair<T> {
    x: T,
    y: T,
}
//关联函数
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
//Pair内部方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
/*
也可以为实现了其他Trait的任意类型有条件的实现某个Trait
为满足Trait Bound的所有类型上实现Trait叫做覆盖实现(blanketimplementations)
例子:
Rust里的string.rs源码
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Display + ?Sized> ToString for T {
    #[inline]
    default fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut formatter = core::fmt::Formatter::new(&mut buf);
        // Bypass format_args!() to avoid write_str with zero-length strs
        fmt::Display::fmt(self, &mut formatter)
            .expect("a Display implementation returned an error unexpectedly");
        buf
    }
}
*/
