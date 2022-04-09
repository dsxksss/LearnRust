/*
Rust 组织管理
对于一个工程来讲，组织代码是十分重要的。

Rust 中有三个重要的组织概念：箱、包、模块。
箱（Crate）
"箱"是二进制程序文件或者库文件，存在于"包"中。
"箱"是树状结构的，它的树根是编译器开始运行时编译的源文件所编译的程序。
注意："二进制程序文件"不一定是"二进制可执行文件"，只能确定是是包含目标机器语言的文件，文件格式随编译环境的不同而不同。

包（Package）
当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。
一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。

模块（Module）
对于一个软件工程来说，我们往往按照所使用的编程语言的组织规范来进行组织，组织模块的主要结构往往是树。Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
这些先进的语言的组织单位可以层层包含，就像文件系统的目录结构一样。Rust 中的组织单位是模块（Module）。
*/

use rust_use::_funtion_a;
use rust_use::f_add::z_add;

//导入外部package例子:
// use std::io;
// use std::io::Write;
//如果要导入多个同package里的内容,可以在use后面使用{}号来使用嵌套路径
use std::io::{self, Write}; //写法一:导入父级内容及此父级的子级内容//self表示自己io本身
use std::{cmp::Eq, io::Read}; //写法二:导入不同的父子级内容

//也可以利用as关键字来给路径在此文件内重新命名
use std::io::BufRead as buf_read; //别名buf_read

//通配符*
/*
使用*可以把路径中所有的公共条目都引入到作用域
注意:谨慎使用
应用场景:
--测试。将所有被测试代码引入到tests模块
--有时被用于预导入(prelude)模块
*/

fn main() {
    println!(
        "my path are rust_use/f_add/z_add/add_some_data return value is {}",
        z_add::add_some_data()
    );
    println!(
        "my path are rust_use/_funtion_a return value is {}",
        _funtion_a()
    );
    println!("Hello, world!");
}
