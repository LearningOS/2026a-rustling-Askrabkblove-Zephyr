toml示例：package 包名（描述包是谁） dependencies 依赖的第三方库

```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.3"
```

当我们引入了一个外部依赖后，Cargo 将从 *registry* 上获取所有依赖所需的最新版本，这是一份来自`[Crates.io]`(https://crates.io/) 的数据拷贝。Crates.io 是 Rust 生态环境中开发者们向他人贡献 Rust 开源项目的地方。

在更新完 registry 后，Cargo 检查 `[dependencies]` 表块并下载缺失的 crate 。

### [创建工作空间](https://rustwiki.org/zh-CN/book/ch14-03-cargo-workspaces.html#创建工作空间)

**工作空间** 是一系列共享同样的 *Cargo.lock* 和输出目录的包。让我们使用工作空间创建一个项目 —— 这里采用常见的代码以便可以关注工作空间的结构。有多种组织工作空间的方式；我们将展示一个常用方法。我们的工作空间有一个二进制项目和两个库。二进制项目会提供主要功能，并会依赖另两个库。一个库会提供 `add_one` 方法而第二个会提供 `add_two` 方法。这三个 crate 将会是相同工作空间的一部分。让我们以新建工作空间目录开始：

```text
$ mkdir add
$ cd add
```

接着在 *add* 目录中，创建 *Cargo.toml* 文件。这个 *Cargo.toml* 文件配置了整个工作空间。它不会包含 `[package]` 或其他我们在 *Cargo.toml* 中见过的元信息。相反，它以 `[workspace]` 部分作为开始，并通过指定 *adder* 的路径来为工作空间增加成员，如下会加入二进制 crate：

文件名: Cargo.toml

```toml
[workspace]

members = [
    "adder",
]
```

接下来，在 *add* 目录运行 `cargo new` 新建 `adder` 二进制 crate：

```text
$ cargo new adder
     Created binary (application) `adder` project
```

到此为止，可以运行 `cargo build` 来构建工作空间。*add* 目录中的文件应该看起来像这样：

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

工作空间在顶级目录有一个 *target* 目录；`adder` 并没有自己的 *target* 目录。即使进入 *adder* 目录运行 `cargo build`，构建结果也位于 *add/target* 而不是 *add/adder/target*。工作空间中的 crate 之间相互依赖。如果每个 crate 有其自己的 *target* 目录，为了在自己的 *target* 目录中生成构建结果，工作空间中的每一个 crate 都不得不相互重新编译其他 crate。通过共享一个 *target* 目录，工作空间可以避免其他 crate 多余的重复构建。

### [在工作空间中创建第二个 crate](https://rustwiki.org/zh-CN/book/ch14-03-cargo-workspaces.html#在工作空间中创建第二个-crate)

接下来，让我们在工作空间中指定另一个成员 crate。这个 crate 位于 *add-one* 目录中，所以修改顶级 *Cargo.toml* 为也包含 *add-one* 路径：

文件名: Cargo.toml

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

接着新生成一个叫做 `add-one` 的库：

```text
$ cargo new add-one --lib
     Created library `add-one` project
```

现在 *add* 目录应该有如下目录和文件：

```text
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

在 *add-one/src/lib.rs* 文件中，增加一个 `add_one` 函数：

文件名: add-one/src/lib.rs

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

现在工作空间中有了一个库 crate，让 `adder` 依赖库 crate `add-one`。首先需要在 *adder/Cargo.toml* 文件中增加 `add-one` 作为路径依赖：

文件名: adder/Cargo.toml

```toml
[dependencies]

add-one = { path = "../add-one" }
```

cargo并不假定工作空间中的Crates会相互依赖，所以需要明确表明工作空间中 crate 的依赖关系。

接下来，在 `adder` crate 中使用 `add-one` crate 的函数 `add_one`。打开 *adder/src/main.rs* 在顶部增加一行 `use` 将新 `add-one` 库 crate 引入作用域。接着修改 `main` 函数来调用 `add_one` 函数，如示例 14-7 所示。

文件名: adder/src/main.rs

```rust
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

示例 14-7：在 `adder` crate 中使用 `add-one` 库 crate

在 *add* 目录中运行 `cargo build` 来构建工作空间！

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68 secs
```

为了在顶层 *add* 目录运行二进制 crate，需要通过 `-p` 参数和包名称来运行 `cargo run` 指定工作空间中我们希望使用的包：

```text
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

这会运行 *adder/src/main.rs* 中的代码，其依赖 `add-one` crate