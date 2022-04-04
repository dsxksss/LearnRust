fn main() {
    println!("Hello, world!");
    println!(
        "another_function  return value is {}",
        another_function(50, 100)
    );
}

fn another_function(x: i32, y: i32) -> i32 {
    x + y
}
