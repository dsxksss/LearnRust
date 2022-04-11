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
}
