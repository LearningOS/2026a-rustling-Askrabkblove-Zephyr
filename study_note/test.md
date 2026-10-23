| 宏                 | 作用                          |
| :----------------- | :---------------------------- |
| `assert!(cond)`    | `cond` 为 `false` 时 panic    |
| `assert_eq!(a, b)` | `a != b` 时 panic（比较相等） |
| `assert_ne!(a, b)` | `a == b` 时 panic（比较不等） |

`assert_eq!` / `assert_ne!` 在失败时会打印出两边的实际值，调试更方便。

## `#[derive(Debug, PartialEq, Eq)]`

这是自动派生 trait：`PartialEq` trait 可以比较一个类型的实例以检查是否相等，并开启了 `==` 和 `!=` 运算符的功能。

派生的 `PartialEq` 实现了 `eq` 方法。当 `PartialEq` 在结构体上派生时，只有*所有* 的字段都相等时两个实例才相等，同时只要有任何字段不相等则两个实例就不相等。当在枚举上派生时，每一个成员都和其自身相等，且和其他成员都不相等。

- **`Debug`**：允许用 `{:?}` 打印，方便调试和测试失败时输出
- **`PartialEq`**：允许用 `==`、`!=` 比较两个 `DivisionError` 是否相等
- **`Eq`**：表示这是“完全等价关系”，是 `PartialEq` 的更强版本
- `Eq` 要求所有字段也实现 `Eq`。这里 `NotDivisibleError` 必须也实现 `Eq`，否则编译失败。

## Rust 默认会"改名"（mangle）

Rust 默认把函数名**修饰（mangle）**成包含模块路径、类型信息的长串，例如：

rust

```
mod Foo {
    fn my_demo_function(a: u32) -> u32 { a }
}
```



编译后的符号名**不是** `my_demo_function`，而可能是：

text

```
_ZN3Foo15my_demo_function17h1234567890abcdefE
```



（实际格式随编译器版本变，大意是 `_ZN` + 模块名 + 函数名 + 哈希 + `E`。）

`no_mangle` 字面意思"**不要修饰**"。加上后，Rust **不再改名**，符号名就是函数原名：

text

```
my_demo_function
```



这就是"**让定义导出为 `my_demo_function` 符号**"的意思：

- **定义**：`Foo::my_demo_function` 这个函数的实现。
- **导出**：在编译产物的符号表里，以 `my_demo_function` 这个名字对外可见。
- **符号**：链接器用来匹配的名字。
