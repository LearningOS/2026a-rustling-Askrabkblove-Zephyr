// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

// Rust 在与 C/C++ 以及其他静态编译语言共享 FFI 接口方面能力很强，甚至可以在代码自身内部进行链接！它是通过 `extern` 块实现的，就像下面的代码那样。
//
// `extern` 关键字后面的短字符串表示被外部导入的函数将遵循哪种 ABI。在本练习中，使用的是 "Rust"，而其他变体也存在，比如标准 C ABI 的 "C"、Windows ABI 的 "stdcall"。
//
// 外部导入的函数在 `extern` 块中声明，以分号结束签名（而不是花括号）。一些属性可以应用到这些函数声明上，以修改链接行为，例如 `#[link_name = ".."]` 用于修改实际的符号名。
//
// 如果你想把你的符号导出到链接环境，`extern` 关键字也可以标记在函数定义前面，并附带相同的 ABI 字符串说明。Rust 函数的默认 ABI 就是字面上的 "Rust"，所以如果你想链接纯 Rust 函数，整个 `extern` 词都可以省略。
//
// Rust 默认会对符号进行名称修饰（mangle），就像 C++ 那样。要抑制这种行为、让这些函数能按名字被寻址，可以应用 `#[no_mangle]` 属性。
//
// 在本练习中，你的任务是让测试用例能够调用模块 `Foo` 中的 `my_demo_function`。`my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用同一个函数。
//
// 你**不应**修改任何现有代码，除了添加两行属性。

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    // 我在代码里叫它 my_demo_function_alias
    // 但实际要找的符号是 my_demo_function。
    // 即：代码里的名字 和 链接时找的符号名 解耦。#[link_name] 指定后者。
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
     #[no_mangle] // 让定义导出为 my_demo_function 符号。
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        // 外部导入的函数默认是不安全（unsafe）的，因为来源是其他语言、不可信。你可以把它们包装进安全的 Rust API 中，以减轻调用者的负担。
        //
        // SAFETY（安全性说明）：我们知道这些函数是某个安全 Rust 函数的别名。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
