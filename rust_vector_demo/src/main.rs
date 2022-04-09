//使用vector配合enum来存储多种数据类型
//(条件:必须提前知道)除非使用trait对象
//--Enum的变体可以附加不同类型的数据
//--Enum的变体定义在同一个enum类型下
//例子:

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
