模块系统的第一部分，我们将介绍包和 crate。crate 是一个二进制项或者库。*crate root* 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块（我们将在[“定义模块来控制作用域与私有性”](https://rustwiki.org/zh-CN/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)一节深入解读）。*包*（*package*）是提供一系列功能的一个或者多个 crate。一个包会包含有一个 *Cargo.toml* 文件，阐述如何去构建这些 crate。

- **绝对路径**（*absolute path*）从 crate 根部开始，以 crate 名或者字面量 `crate` 开头。
- **相对路径**（*relative path*）从当前模块开始，以 `self`、`super` 或当前模块的标识符开头。
- 我们还可以使用 `super` 开头来构建从父模块开始的相对路径。

我们用关键字 `mod` 定义一个模块，指定模块的名字（在示例中为 `front_of_house`），并用大括号包围模块的主体。

使用 `use` 将两个同名类型引入同一作用域这个问题还有另一个解决办法：在这个类型的路径后面，我们使用 `as` 指定一个新的本地名称或者别名。
