fn main() {
    //loop循环
    let mut lp = 0;
    //循环也是一个表达式，可以写成这样
    let result = loop {
        //我是个死循环
        println!("i am loop");
        lp += 1;
        if lp > 5 {
            break lp;
        }
    };

    let mut number = 3;
    //while循环
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("loop result is {}", result);

    //for循环,这里用遍历数组做例子
    let a = [10, 20, 30, 40, 50];
    //iter返回一个可以用来遍历的类型
    for element in a.iter() {
        //这里的element并不是将值复制出来，而是一个指针
        println!("the value is: {}", element);
    }

    //使用for循环实现上面的while同功能例子
    //这里的1..4表示的是从1开始到4前一位之间的数,rev()函数是反转这之间的数
    for sum in (1..4).rev() {
        println!("{}!", sum);
    }

    println!("Hello, world!");
}
