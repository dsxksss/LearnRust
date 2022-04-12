use std::io;
use std::io::Read;
use std::{io::ErrorKind, panic};

/*
Result枚举
rust中我们一般用此枚举类型来处理一些可以恢复的错误
枚举原型:
enum Result<T,E>{
    Ok(T),
    Err(E),
}
T:操作成功的情况下，Ok变体里返回的数据的类型
E:操作失败的情况下，Err变体里返回的错误的类型
例子:
*/
use std::fs::File;
fn main() {
    //处理Result的一种方式:match表达式
    //--和Option枚举一样，Result及其变体也是有Prelude(预导入模块)带入作用域的，
    //使用时可以不用加上 :: 及 前缀
    //例子:
    //let _f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file, //因为返回一个file数据类型，所以要给定一个存储空间
    //     Err(e) => {
    //         panic!("open file error !!! msg:{:?}", e) //记得好好利用panic!警告
    //     }
    // };

    //可以优化上面的例子，匹配不同的错误
    //例子:
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        //打开文件失败之后我们针对err里的错误，再进行match
        Err(e) => match e.kind() {
            //kind是为了获得此错误的io错误
            //ErrorKind是标准库里的东西，是专门用来判断程序的FileIOError的
            //下面这个功能是判断文件是否没找到，如果没找到则创建一个相同名字的文件
            //因为创建文件本身的话也有可能会失败，所以还要判断是否已经创建成功，返回一个Result
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file : {:?}", e),
            },
            //如果不是遇到的FileIOError情况的话则panic!为其他的情况
            //other_error是自定义名字，可以更改
            other_error => panic!("Error opening the file : {:?}", other_error),
        },
    };

    /*
    上例中使用了很多match
    虽然match很有用，但是很 “原始”
    闭包(closure)。Result<T,E>有很多方法:
    --它们接受闭包作为参数
    --使用match实现
    --使用这些方法会让代码更简洁
    再优化上例例子:
    */
    //作用和上面那个例子是一模一样的
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file : {:?}", error);
            })
        } else {
            panic!("Error opening file : {:?}", error);
        }
    });

    /*
    unwrap方法
    -unwrap:  match表达式的一个快捷方法:
    --如果Result结果是Ok，则返回Ok里面的值
    --如果Result结果是Err，则调用一个panic!，
    --缺点是错误信息不可以自定义
    例子:
    */
    // let _f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file, //因为返回一个file数据类型，所以要给定一个存储空间
    //     Err(e) => {
    //         panic!("open file error !!! msg:{:?}", e) //记得好好利用panic!警告
    //     }
    // };

    //功能相等于上面这个例子，代码简洁了许多，这是Result内置的快捷方法
    let _f = File::open("hello.txt").unwrap();

    /*
    expect
    expect和unwrap方法类似，但是可以指定错误信息
    例子:
    */
    let _f = File::open("hello.txt").expect("file opening Error!");

    let _result = read_username_from_file();
}
/*
传播错误
-在函数中处理错误
-将错误返回给调用者
*/
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
/*
?运算符
？运算符:传播错误的一种快捷方式
-如果Result是Ok:Ok中的值接受表达式的结果，然后继续执行程序
-如果Result是Err:Err就是 "整个函数" 的返回值，就像使用了return
利用?运算符优化代码的例子:
*/
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
/*
了解内容:
?与from函数
-Traot std::convert::From上的from函数:
--用于错误之间的转换
-被?所应用的错误，会隐式的被from函数处理
-当?调用from函数时:
--它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
-用于:针对不同错误原因，返回同一种错误类型
--只要每个错误类型实现了转换为所返回的错误类型的from函数
*/

//可以使用?运算符运用链式调用继续优化read_username_from_file()函数:
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
//tip小贴士:?运算符只能用于返回Result、Option、或者是其他实现了‘Try’类型的函数
/*
?运算符与main函数
-main函数返回类型是:()
-main函数的返回类型也可以是:Result<T,E>
-Box<dun Error>是trait对象:
--简单理解为:"任何可能的错误类型"

fn main(){
    let f=File::open("hello.txt")?;   xxx
}
上例main函数因为返回的是()所以不能使用?运算符，
但是main函数支持返回Result类型，例子:
use std::error::Error;
fn main()->Result<(),Box<dyn Error>>{
    let f=File::open("hello.txt")?;
    Ok(())
}
*/
