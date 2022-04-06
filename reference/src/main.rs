fn main() {
    //&在rust里表示引用的意思，和cpp里的引用类似
    //引用在rust里也是默认不可变的，也就表示我利用引用不能去修改被引用里的值的内容
    //如果要使用可变引用,也得和变量那一样加上mut关键字表示我可以去修改被引用的内容

    /*
    引用的规则：
    在任何给定的时刻，只能满足下列条件之一：
        · 一个可变的引用
        · 容易数量不可变的引用
        · 引用必须一直有效
     */
    let mut s = String::from("我是在堆内存上创建的一段字符串");
    dangle(&mut s);
    println!("Hello, world!");
}

fn dangle(s: &mut String) {
    s.push_str(",我是后续在dangleg函数内加上的内容");
    println!("string is :{}, this string lenght is {}", s, s.len());
}
