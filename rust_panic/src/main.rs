/*
Rust错误处理概述
-rust的可靠性:错误处理
大部分情况下:在编译时提示错误，并处理
-错误的分类:
---可恢复的错误
-----例如文件未找到，可再次尝试
---不可恢复的错误
-----bug，例如访问的索引超出范围


Rust没有类似异常的机制
--可恢复错误:Result<T,E>
--不可恢复错误:panic!宏
*/
fn main() {
    /*
    不可恢复的错误与panic!
    当panic!宏执行:
    1-你的程序会打印一个错误信息
    2-展开(unwind)、清理调用栈(Stack)
    3-退出程序

    为应对panic!，展开或中止(abort)调用栈
    -默认情况下，当panic发生:
    --程序展开调用栈(工作量打)
    --·rust沿着调用栈往回走
    --·清理每个遇到的函数中的数据
    -或立即终止调用栈
    --·不进行清理，直接停止程序
    --·内存需要OS进行清理

    如果你想让二进制文件更小，可以设置从“展开”改为“中止”：
    -在Cargo.toml中适当的profile部分设置:
    --·panic='abort'
    例子:

    cargo.toml file : ->
    [profile.release]
    panic = 'abort'
    */
    println!("Hello, world!");

    //panic!("i am a panic Error!")

    /*
    使用panic!产生的回溯信息
    panic!可能出现的在:
    --我们写的代码中
    --我们所依赖的代码中
    可通过调用panic!的函数的回溯信息来定位引起问题的代码
    --通过设置环境变量RUST_BACKTRACE=1之后再运行程序就可得到回溯信息
    --为了获取带有调试信息的回溯，必须启用调试符号(不带 --release)
    例如:cargo run --release(不带)
    */

    /*----panic!补充 */
    //---------------我们应该何时使用panic------------:

    /*
    1、编写实例、原型代码、测试
    -可以使用panic!
    --演示某些概念时:unwrap
    --编写原型代码时:unwrap、expect
    --测试时:unwrap、expect

    2、有时你比编译器掌握更多的信息
    -你可以确定Result就是Ok:unwrap
    例子:
    use std::net::IpAddr;
    fn main(){
        let home="127.0.0.1".parse().umwrap();
    }

    3、错误处理的指导性建议
    -当代码最终可能处于损坏状态时，最好使用panic!
    -损坏状态(Bad state):某些假设、保证、约定或不可变性被打破时可以使用panic!
    --例如:非法的值、矛盾的值或空缺的值被传入代码
    --以及下列中的一条时:
    ---这种损坏状态并不是预期能够偶尔发生的事情。
    ---在此之后，您的代码如果处于这种损坏状态就无法允许
    ---在您使用的类型中没有一个好的方法来将这些信息(处于损坏状态)进行编码

    4、场景建议
    -调用你的代码，传入无意义的参数值时:panic!
    -调用外部不可控代码，返回非法状态，你无法修复:panic!
    -如果失败是可预期的请使用:Result
    -当你的代码对值进行操作，首先应该验证这些值:panic!

    5、为验证创建自定义类型
    例子:
    fn main(){
        loop{
            //...
            let guess = "32"
            let guess:i32 = match guess.trim().parse(){
                Ok(num)=>num,
                Err(_)=>continue,
            };

            if guess < 1 || guess > 100{
                println!("The secret number will be between 1 and 100.");
                continue;
            }
            //...
        }
    }
    -针对这种情况，我们可以创建新的类型，把验证逻辑放在构造实例的函数里。
    例子:
    pub struct Guess{
        value:i32,
    }

    impl Guess{
        pub fn new(value:i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}",value);
             }
             Guess {value}
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    fn main(){
        loop{
            //...
            let guess = "32";
            let guess = match guess.trim().parse(){
                Ok(num)->num,
                Err(_)=>continue,
            };
            let guess = Guess::new(guess);
            ///...
        }
    }
    --getter:返回字段数据
    */
}
