/*
如果你现在正在开发一个图书管理系统，你需要描述两种书的不同属性
(纸质书有索书号，电子书只有 URL)，你可以为枚举类成员添加元组属性描述：
*/

// enum Book {
//     Papery(u32),
//     Electronic(String),
// }

enum Book {
    //如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字：
    Papery(u32),
    //如果你想为属性命名，可以用结构体语法：
    Electronic { url: String },
}
//枚举方法,和struct方法差不多
impl Book {
    fn call(&self) {
        println!("i am function in Book enum");
    }
}

/*
let book = Book::Papery{index: 1001};
虽然可以如此命名，但请注意!
并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在 match 语法中。
*/

/*
match 语法
枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。
基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。
switch 语法很经典，但在 Rust 中并不支持，很多语言摒弃 switch 的原因都是
因为switch 容易存在因忘记添加 break 而产生的串接运行问题，
Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
*/

fn main() {
    let book = Book::Papery(1001);
    let book_path = Book::Electronic {
        url: String::from("url://..."),
    };
    //使用枚举方法
    book.call();

    //Rust 通过 match 语句来实现分支结构。先认识一下如何用 match 处理枚举类：
    match book {
        Book::Papery(i) => {
            println!("Papery book {}", i);
        }
        Book::Electronic { url } => {
            println!("Electronic-book {}", url);
        }
    }

    match book_path {
        Book::Papery(i) => {
            println!("Papery book {}", i);
        }
        Book::Electronic { url } => {
            println!("Electronic-book {}", url);
        }
    }
    //match 块也可以当作函数表达式来对待，它也是可以有返回值的
    //但是所有返回值表达式的类型必须一样！
    println!("Hello, world!");

    /*
        match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、
        字符和字符串切片引用（&str）类型的数据进行分支选择。
        其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。
        对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事
        例外情况用下划线 _ 表示：
        fn main() {
        let t = "abc";
        match t {
            "abc" => println!("Yes"),
            _ => {},
        }
    }
        */
}
