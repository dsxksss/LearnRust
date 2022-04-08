#[derive(Debug)]
enum UsState {
    _Alabama,
    _Akaska,
}

enum Coin {
    //美元面值
    Penny,
    _Nickel,
    _Dime,
    _Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        /*
        绑定值的模式:
        匹配的分支可以绑定到被匹配对象的部分值。因此可以从enum变体中提取值
        */
        Coin::_Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//匹配Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let a = Coin::Penny;
    let b = Coin::_Quarter(UsState::_Akaska);
    let _c = plus_one(Some(5));
    let _d = plus_one(None);
    println!("a value in cents is {}", value_in_cents(a));
    println!("b value in cents is {}", value_in_cents(b));

    //match匹配必须穷举所有的可能
    //如果只想使用其中一个可能值
    //可以使用_通配符:替代其余没列出的值
    //例子:
    let v = 0u8; //因为是0到256种可能
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //可以使用这种方法来省略剩余没被举例出来的可能性
    }

    //if let匹配(一种简单的match,因为只匹配一种可能情况)
    /*
    处理只关心一种匹配而忽略其他匹配的情况
    更少的代码，更少的缩进，更少的模板代码
    放弃了穷举的可能
    可以把if let看作是match的语法糖
    */
    if let 100 = v {
        println!("i am 100");
    }
    //if let 也可以搭配else使用
    else {
        println!("i am others");
    }
}
