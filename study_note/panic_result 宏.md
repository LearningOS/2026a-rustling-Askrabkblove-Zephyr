### [对应 panic 时的栈展开或终止](https://rustwiki.org/zh-CN/book/ch09-01-unrecoverable-errors-with-panic.html#对应-panic-时的栈展开或终止)

当出现 panic 时，程序默认会开始 **展开**（*unwinding*），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接 **终止**（*abort*），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。如果你需要项目的最终二进制文件越小越好，panic 时通过在 *Cargo.toml* 的 `[profile]` 部分增加 `panic = 'abort'`，可以由展开切换为终止。例如，如果你想要在release模式中 panic 时直接终止：

```toml
[profile.release]
panic = 'abort'
```

### [`?` 运算符可被用于返回 `Result` 的函数](https://rustwiki.org/zh-CN/book/ch09-02-recoverable-errors-with-result.html#-运算符可被用于返回-result-的函数)

`?` 运算符可被用于返回值类型为 `Result` 的函数，因为他被定义为与示例 9-6 中的 `match` 表达式有着完全相同的工作方式。`match` 的 `return Err(e)` 部分要求返回值类型是 `Result`，所以函数的返回值必须是 `Result` 才能与这个 `return` 相兼容。

让我们看看在 `main` 函数中使用 `?` 运算符会发生什么，如果你还记得的话其返回值类型是 `()`：

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```





当编译这些代码，会得到如下错误信息：

```text
error[E0277]: the `?` operator can only be used in a function that returns
`Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a
  function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
```

错误指出只能在返回 `Result` 或者其它实现了 `std::ops::Try` 的类型的函数中使用 `?` 运算符。当你期望在不返回 `Result` 的函数中调用其他返回 `Result` 的函数时使用 `?` 的话，有两种方法修复这个问题。一种技巧是将函数返回值类型修改为 `Result<T, E>`，如果没有其它限制阻止你这么做的话。另一种技巧是通过合适的方法使用 `match` 或 `Result` 的方法之一来处理 `Result<T, E>`。

`main` 函数是特殊的，其必须返回什么类型是有限制的。`main` 函数的一个有效的返回值是 `()`，同时出于方便，另一个有效的返回值是 `Result<T, E>`，如下所示：

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

`Box<dyn Error>` 被称为 “trait 对象”（trait object），第 17 章 [“为使用不同类型的值而设计的 trait 对象”](https://rustwiki.org/zh-CN/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) 部分会做介绍。目前可以理解 `Box<dyn Error>` 为使用 `?` 时 `main` 允许返回的 “任何类型的错误”。

现在我们讨论过了调用 `panic!` 或返回 `Result` 的细节，是时候回到他们各自适合哪些场景的话题了。