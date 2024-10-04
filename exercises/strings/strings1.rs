// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
//String：是一种可增长的、堆分配的字符串类型。
//&str：是一个不可变的字符串切片,它可以是一个静态字符串（字面值）或一个 String 的借用。