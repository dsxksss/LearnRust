//Option枚举
//定义于标准库中
//在Prelude(预导入模块)中,这表示我们可以直接使用Option
//描述了:某个值可能存在(某种类型)或不存在的情况

//Rust没有Null
//其他语言中:
//Null是一个值,它表示"没有值"

//rust中类似Null概念的枚举-Option<T>
//标准库中的Option的定义:
/*
enum Option<T>{
    Some(T),
    None,
}
*/
fn main() {
    let _some_number = Some(5);
    let _some_string = Some("A String");
    //在你还不知道给某个变量赋予初始值时,你可以使用此方法来说明它的值为空。(必须告知类型)
    let _absent_number: Option<i32> = None;
    println!("Hello, world!");
}
//Option<T>比其他编程语言里的Null要好在哪里？
//Option<T>和T是不同的类型,不可以把Option<T>直接当成T
//例子:
/*
let x:i8=5;
let y:Option<i8> = Some(5);
let sum = x+y;//这里会报错:无法将i8类型的值与Option<i8>类型的值相加
*/
//若想使用Option<T>中的T,必须将它转换为T
