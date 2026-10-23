// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
// Rust 中的 unsafe 充当一份契约。
//
// 当 unsafe 标记在某个条目声明上（比如函数、trait 等）时，它同时声明了一份契约。然而，契约的内容无法仅用一个关键字表达。
// 因此，你需要在条目的文档注释中的 # Safety 段落里手动写明这份契约。
//
// 当 unsafe 标记在由花括号包围的代码块上时，它声明的是对某份契约的遵守，例如某个指针参数的有效性、某块内存地址的所有权。
// 然而，和上面一样，你仍然需要在代码块的注释中说明这份契约是如何被遵守的。
//
// 注意：所有注释都是为了代码的可读性和可维护性，而 Rust 编译器把代码健全性的信任交给你自己！
// 如果你无法证明自己代码的内存安全和健全性，请退一步，改用安全代码！

//
// 执行 rustlings hint tests5，或使用 hint 观察子命令来获取提示。
// 注意：所有注释都是为了代码的可读性和可维护性，而Rust编译器将代码健全性的信任交给你自己！如果你无法证明自己代码的内存安全和健全性，请退一步，改用安全代码！

//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    // TODO：在下方代码块中填写你的安全说明（safety notice），使其与你代码的行为以及本函数的契约相符。你可以参考下方测试中的注释作为格式范例。
    unsafe {
        //todo!("Your code goes here")
        // *mut u32 是 Rust 的可变裸指针（mutable raw pointer）类型。拆开看：
        // *mut	可变的裸指针（raw pointer）
        //  u32	指针指向的类型
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        // as *mut u32 把可变引用强制转换成裸指针（raw pointer）。
        // as usize 把指针转成整数（指针本质就是一个内存地址，可以用整数表示）。
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
