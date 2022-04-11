/*
HashMap<K,V>
键值对的形式存储数据，一个键(Key)对应一个值(Value)
Hash函数:决定如何在内存中存放K和V
适用场景:通过K(任何类型)来寻找数据，而不是通过索引
*/

fn main() {
    /*
    创建HashMap
    创建空HashMap:new()函数 (必须指定类型或者在代码下行添加数据)
    添加数据:insert()方法
    例子:
    */
    use std::collections::HashMap;
    //指定类型方法：
    let mut _scores1: HashMap<String, i32> = HashMap::new();
    //后续添加数据方法：
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Yellow"), 50);

    /*
    HashMap用的比较少，不在Prelude(预导入模块)中,所以得手动导入
    标准库对其支持较少，没有内置的宏来创建HashMap
    数据存储在heap上
    同构的。一个HashMap中:
    --所有的K必须是同一种类型
    --所有的V必须是同一种类型
    */

    /*
    另一种创建HashMap的方式:collect方法
    --在元素类型为Tuple的Vector上适用collect方法，可以组建一个HashMap:
    ---要求Tuple有两个值:一个作为K，一个作为V
    ---collect方法可以把数据整合成很多种集合类型，包括HashMap
    ----返回值需要显式指明类型
    例子:
    */
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    //zip方法会返回一个元组
    let _scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    /*
    HashMap和所有权
    对于实现了Copy trait的类型(例如i32)，值会被复制到HashMap中
    对于拥有所有权的值(例如String)，值会被移动，所有权会转移给HashMap
    例子:
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{}:{}", field_name, field_value);

    //如果将值的引用插入到HashMap,值本身不会移动
    //在HashMap有效的期间，被引用的值必须保持有效
    //例子:
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    //如果只传引用进去，则不会改变原来值的所有权
    map.insert(&field_name, &field_value);
    println!("{}:{}", field_name, field_value);

    /*
    访问HashMap中的值
    get方法
    --参数:K
    --return:Option<&V>
    例子:
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get("Blue");
    match score {
        Some(s) => println!("{}", s),
        None => println!("team not exist"),
    }

    /*
    遍历HashMap
    也是利用for循环
    例子:
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("for -> {}:{}", k, v);
    }

    /*
    更新HashMao<K,V>
    HashMap大小可变
    每个K同时只能对应一个V
    更新HashMap中的数据:
    ---K已经存在，对应一个V
    ----替换现有的V
    ----保留现有的V，忽略心的V
    ----合并现有的V和新的V
    ---K不存在
    ----添加一对K，V
    */

    /*
    替换现有的V
    如果向HashMap插入一对KV，然后再插入同样的K，
    但是不同的V，那么原来的V会被替换掉
    例子:
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    /*
    只在K不对应任何值的情况下，才插入V
    ---entry方法:检查指定的K是否对应一个V
    ----参数为K
    ----返回enum Entry:代表值是否存在

    ---Entry的or_insetr()方法:
    --返回:
    ----如果K存在，返回到对应的V的一个可变引用
    ----如果K不存在，将方法参数作为K的新值插进去，返回到这个值的可变引用
    例子:
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    //or_insert如果不存在此K则插入某个V
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    //基于现有V来更新V

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        //使用*解引用符来操作被引用的值
        *count += 1;
    }
    println!("{:#?}", map);

    /*
    Hash函数
    -默认情况下，HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务攻击(DOS)攻击
    --不是可用的最快的Hash算法
    --但具有更好的安全性
    -可以指定不同的Hasher来切换到另一个函数
    --hasher是实现BuildHasher trait的类型
    */
}
