//创建一个新的字符串
/*
很多Vec<T>的操作都可以用于String
&String::new()函数可以创建一个空的字符串
*/

fn main() {
    //使用初始值来创建String:
    //---to_string()方法，可用于实现Dispaly trait的类型，包括字符串字面值
    //---String::from()函数，从字面值创建String
    //上面这两个函数或方法的功能都是一样的，看自己喜好选择
    //他们都是UTF-8编码存储或显示的

    //利用to_string方法来将字符串字面量给转换成String类型
    //例子:
    let mut data1 = "initial contents".to_string();
    println!("from to data1 string: {}", &data1);

    //利用String::from函数来创建一个String类型字符串
    //例子:
    let mut data2 = String::from("initial contents");
    println!("from to data2 string: {}", &data2);

    //更新String
    //push_str()方法:把一个字符串切片附加到String(记得加上mut关键字)
    //push_str()方法并不会获得使用者的所有权
    //例子:
    data1.push_str("我是附加上去的字符串");
    println!("{}", &data1);

    //push()方法:把单给字符附加到String
    //例子:
    data2.push('Z');
    data2.push('X');
    data2.push('K');
    println!("{}", &data2);

    //可以使用+号运算符将两个字符串拼接起来
    //注意+号前面是String类型，+后面的是&String类型的
    println!("{}", data1 + &data2);

    //利用+号运算符拼接起来的字符串之后，+号前面的字符串会转移所有权，所有之后都用不了了
    //但是+号后面的值却可以再使用
    //例子: println!("{}", data1);

    /*
    原因是:
    ---使用了"类似"这个签名的方法fn add(self,s:&str)->String{...}
    --标准库中的add方法使用了泛型，所以是类似的一个签名方法
    --只能把&str添加到String
    --解引用强制转换了&String至&str(deref coercion)
    */

    //除了+号运算符来拼接字符串，也可以使用更灵活的方式:format!宏来使用
    //和println!()类似，但是format!会返回字符串
    //format!不会获得任何参数的所有权
    let s = format!("{}-{}", data2, data2);
    println!("{}", s);

    /*
    对String按索引的形式进行访问
    --按索引语法访问String的某部分，会报错(例子)
    let s1 = String::from("hello");
    let h = s1[0];      xxx
    --Rust的字符串不支持索引语法访问


    String类型的内部表示
    --String是一个对Vec<u8>类型的包装出来的产物
    ---len()方法:返回String类型变量的字节数
    */
    let len = String::from("hola").len();
    println!("len length is {}", len); //4
                                       //如果是汉字的话就会占两个字节
                                       //它们可以称作Unicode 标量值

    /*
    字节、标量值、字形簇
    Bytes、Scalar Values、Grapheme Clusters
    --rust有三种看待字符串的方式:
    --·字节
    --·标量值
    --·字形簇(最接近所谓的"字母")
    */

    //例子1:以字节形式看待
    let w1 = "你好你是？";
    for i in w1.bytes() {
        println!("{}", i);
    }

    //例子2:以Unicode标量值形式看待
    for i in w1.chars() {
        println!("{}", i);
    }

    //例子3:以字形簇的形式看待(这里要使用第三方库去显示就不用了)
    //...

    //Rust不允许对String进行索引的最后一个原因:
    //--索引操作应消耗一个常量时间(0(1))
    //而String无法保证:需要遍历所有内容，来确定有多少个合法的字符

    /*
    切割String
    --可以使用[]和一个范围来创建字符串的切片(例子)
    ---必须谨慎使用
    ---如果切割时跨越了字符串边界，程序就会panic
    --()
    */
    let w2 = "你好世界";
    let s = &w2[0..6];
    println!("{}", s);

    /*
    遍历String的方法
    在rust有三种看待字符串的方式提到了:
    对于标量值:char()方法
    对于字节:bytes()方法
    */

    /*---------------总结--------------
    String不简单
    --rust选择正确处理String数据作为所有Rust程序的默认行为
    ---程序员必须在处理UTF-8数据之前投入更多的精力
    --可防止在开发后期处理涉及非ASCII字符的错误
    */
}
