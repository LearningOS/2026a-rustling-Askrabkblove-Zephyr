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
