use std::string;

//struct结构体
//使用struct关键字,并未整个struct命名
//在花括号内,为所有Field(字段)定义名称和类型
//例如:
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    //创建一个User结构体实例,可以不按顺序创建实例
    //因为struct默认也是不可变的,如果后续需要再次改变内容,记得加上mut关键字
    //一旦struct的实例是可变的,那么实例中所有的字段都是可变
    let user1 = User {
        email: String::from("abc@qq.com"),
        name: String::from("dsxk"),
        active: true,
        sign_in_count: 556,
    };

    println!("i am user1 name data : {}", user1.name);

    //struct更新语法
    //当你想基于某个struct实例来创建一个新实例的时候,可以使用struct更新语法
    //例子:
    let user2 = User {
        email: String::from("def@qq.com"),
        name: String::from("ssss"),
        ..user1 //像这样把user1里的active和sign_in_count值保留到user2这个实例里
    };

    //Tuple struct
    //可定义类似tuple的struct,叫做tuple struct
    //·Tuple struct整体有名,但里面的元素没有名
    //·适用:想给整个tuple起名,并让它不同于其他tuple,而且又不需要给每个元素起名
    //定义tuple struct:使用struct关键字,后边是名字,以及里面元素的类型
    //例子:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //black和origin是不同的类型,是不同的tuple struct的实例
    //访问的话和tuple(元组)类似,可以使用结构方法访问,也可以使用.运算符访问

    //Unit-Like Struct(没有任何字段的结构体)
    //可以定义没有任何字段的struct,叫做Unit-link struct(因为与(),单元类型类似)
    //一般适用在需要某个类型上实现某个trait(接口),但是在里面又没有想要存储的数据

    //struct数据的所有权
    //上面定义的User struct里的字段使用了String而不是&str:
    //·该struct实例拥有其所有的数据
    //·只要struct实例是有效的,那么里面的字段数据也是有效的

    //struct里可以存放引用,但这需要使用生命周期
    //·生命周期保证只要struct实例是有效的,那么里面的引用也是有效的
    //·如果struct里面存储引用,而不使用生命周期的话,就会报错
}
//struct允许作为函数的返回值
//字段初始化简写:当字段名字和字段值对应的变量名相同时可以使用字段初始化简写
//例子:
fn build_user(email: String, user_name: String) -> User {
    User {
        email,
        name: user_name,
        active: true,
        sign_in_count: 1,
    }
}
