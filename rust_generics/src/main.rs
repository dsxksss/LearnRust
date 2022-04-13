/*
泛型
-泛型可以提高代码复用能力
--处理重复代码的问题
-泛型是具体类型或其他属性的抽象代替:
--你编写的代码不是最终的代码，而是一种模板，里面有一些“占位符”
--编译器在编译时将“占位符”替换为具体的类型
例如:fn largest<T>(list:&[T])->T{...}
T代表泛型的类型参数
类型参数可自定，惯例为大写字母T
--很短，通常是一个字母代表
--一般使用CamelCase(驼峰命名法)
--T:type的缩写
*/

/*
函数定义中的泛型
-泛型函数
--参数类型
--返回类型
例子:
*/
//读取一段数据取数据中的最大值
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

/*
struct定义中的泛型
基本上在字段里定义泛型
-可以使用多个泛型的类型参数
--但是太多类型参数:你的代码需要重组为多个更小的单元才易于维护或阅读
例子:
*/
//这种情况赋值时也会必须需要同类型
//例如这种情况就不行:let a=Pint{x:5,y:1.5}; xxx
struct Point<T> {
    x: T,
    y: T,
}
//除非使用两个泛型参数才可以赋予不同类型的值
struct Point2<T, F> {
    x: T,
    y: F,
}

/*
Enum定义中的泛型
-可以让枚举的变体持有泛型数据类型、也可以不持有
--例如Option<T>,Result<T,E>
例子:
*/
enum _Option<T> {
    Some(T),
    None,
}
enum _Result<T, E> {
    Ok(T),
    Err(E),
}

/*
方法定义中的泛型
为struct或enum实现方法的时候，可在定义中使用泛型
注意:
-把T放在impl关键字后，表示在类型T上的实现方法
--例如:impl<T> Point<T>
-只针对具体类型实现方法(其余类型没实现方法):
--例如:impl Point<i32>
例子:
*/

//我们需要在impl关键字后面加上泛型参数，
//这样写表示它是针对这个泛型T而不是某个具体的类型来实现的x这个方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//在针对具体的方法类型是impl后面就可以不用加上泛型参数
//x1这个方法只有在i32这个具体的类型上面才有，而其他的Point<T>的类型则没有x1这个方法
impl Point<i32> {
    fn _x1(&self) -> &i32 {
        &self.x
    }
}

//struct里的泛型类型参数可以和方法的泛型类型参数不同
//例子:
impl<T, F> Point2<T, F> {
    //方法类型和结构体类型不一样，这样是允许的
    //方法参数other的类型也是一个Point2，它的类型也是和原定义的也是可以不一样的
    //当是方法参数的类型必须也是在泛型方法内的参数类型
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

/*
泛型代码的性能
-使用泛型的代码和使用具体的类型的代码运行速度是一样的
-单态化(monomorphization):
--在编译时将泛型替换为具体类型的过程
*/

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 4.7 };
    println!("integer.x={} , integer.y={}", integer.x, integer.y);
    println!("float.x={} , float.y={}", float.x, float.y);

    //这种情况才可以赋予不同类型的值、当然也可以是相同的类型
    let integer2 = Point2 { x: 7, y: 10 };
    let float2 = Point2 { x: 1.5, y: 4 };
    println!("integer2.x={} , integer2.y={}", integer2.x, integer2.y);
    println!("float2.x={} , float2.y={}", float2.x, float2.y);

    //方法泛型的使用
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    //struct里的泛型类型参数可以和方法的泛型类型参数不同中的使用方法
    let p1 = Point2 { x: 5, y: 4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {} , p3.y = {}", p3.x, p3.y);
}
