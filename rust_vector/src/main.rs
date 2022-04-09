/*
使用vector存储多个值
Vec<T>,叫做vector
--由标准库提供
--可存储多个值
--只能存储相同类型的数据
--值在内存中连续存放
*/

fn main() {
    /*
    创建Vector
    --例子: let v: Vec<i32> = Vec::new();
    常见的创建Vector语法是:利用vec!宏来实现
    */
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    //用push方法来向Vector里添加元素
    v.push(4);
    println!("{:?}", v);
    //与其他任何struct一样，当Vector离开作用域后
    //--它就被清理掉了
    //--它所有的元素也会被一同清理掉

    //读取Vector的元素
    /*
                索引 vs get处理访问越界
    索引:返回:panic,程序直接崩溃
    get:返回None,可以用match来处理越界情况,程序不会出现崩溃
    */
    //--两种方式可以引用Vector里的值:

    //---索引
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    //---get方法
    match v.get(4) {
        Some(a) => println!("The third element is {}", a),
        None => println!("There is no third element"),
    }

    //利用for循环来修改vector中的元素
    for i in &mut v {
        *i += 50;
    }

    //利用for循环来访问vector中的每个元素值
    for k in &v {
        println!("{}", k);
    }
}
