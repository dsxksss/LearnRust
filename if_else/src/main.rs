fn main() {
    //如果使用了多余一个else if,那么最好使用match来重构代码
    let number = 3;
    if number % 4 == 0 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //因为if是一个表达式，所以也可以放在let关键字的右边
    let condition = true;
    let number = if condition { 5 } else { 6 }; //类似于其他语言里 三元运算符(三目运算符)
    println! {"{}",number};
    println!("Hello, world!");
}
