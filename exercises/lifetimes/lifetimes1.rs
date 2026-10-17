// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// Rust 编译器需要知道如何检查传入的引用是否有效，
// 以便在引用有在自身被使用之前就离开作用域的风险时，
// 提醒程序员。记住，引用是借用，并不拥有它们自己的数据。
// 如果它们的所有者离开了作用域，会怎样呢？
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
