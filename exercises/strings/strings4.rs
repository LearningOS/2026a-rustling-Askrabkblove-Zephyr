// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // 字符串字面量本身就是 &str
    string("red".to_string()); // to_string() 返回 String
    string(String::from("hi")); // String::from 返回 String
    string("rust is fun!".to_owned());// to_owned() 返回 String
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));// format! 返回 String
    string_slice(&String::from("abc")[0..1]); // 对 String 切片得到 &str
    string_slice("  hello there ".trim()); // trim() 返回 &str（借用原串）
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace 返回 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());// to_lowercase() 返回 String

}
