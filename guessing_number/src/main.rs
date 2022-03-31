use std::io; //导入标准库io

use std::cmp::Ordering; //导入Ordering枚举类型

use rand::Rng; //::号表示后面的东西是前面这个东西的关联函数
fn main() {
    println!("这是一个猜数的游戏,结果会生成一个从1-100以内的随机数,请输入你要猜测的数:");
    let secret_number = rand::thread_rng().gen_range(1, 101); //利用rand包生成一个随机数
    println!("神秘数字是{}", secret_number);

    loop {
        //无限循环
        let mut number = String::new(); //生成一个空字符串
        io::stdin() //使用io标准库中的stdin函数
            //read_line的意思是读取一行用户输入的信息,存储到number的引用地址,因为是可变的所以要加上mut关键字
            .read_line(&mut number)
            //发生读取错误时显示的报错信息
            .expect("无法读取您输入的这行信息");
        //trim:删除前后多余的空格和其他特殊符号,parse:转换成int类型(要手动给定类型)
        //这里的代码优化了一下，使用了match关键字来处理用户输入的结果判断
        let number: u32 = match number.trim().parse() {
            Ok(num) => num, //num表示用户输入的值，没问题所以就返回了num，=>也可以理解为js里的=>意思差不多
            Err(_) => continue, //_表示我并不关心用户输入的值，因为出现了错误，所以利用continue关键字跳出本次循环，继续下次循环
        };
        match number.cmp(&secret_number) {
            //类似于此条件表达式  number > < == secret_number
            //比较一个数，返回一个Ordering枚举类型。
            //less:小于,equal:等于,greater:大于
            //根据返回的枚举类型来执行相应的代码
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
