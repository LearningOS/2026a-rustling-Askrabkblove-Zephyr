// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {// 第一个分支（处理"什么都不传"）
        println!("Check out my macro!");
    };
    // $( ... ),*	把里面的东西按逗号分隔，重复 0 次或多次
    ($($val:expr),*) => { // $val 给匹配到的东西起个名字，叫 val, :expr	要求它必须是一个表达式（数字、变量、算式都行）
        $( // $( ... )* 是关键：你匹配到几个 $val，就把里面的 println! 重复生成几遍，每个 $val 生成一条。
            println!("Look at this other macro: {}", $val);
         )*
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
