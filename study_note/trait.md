### [定义 trait](https://rustwiki.org/zh-CN/book/ch10-02-traits.html#定义-trait)

一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

例如，这里有多个存放了不同类型和属性文本的结构体：结构体 `NewsArticle` 用于存放发生于世界各地的新闻故事，而结构体 `Tweet` 最多只能存放 280 个字符的内容，以及像是否转推或是否是对推友的回复这样的元数据。

我们想要创建一个多媒体聚合库用来显示可能储存在 `NewsArticle` 或 `Tweet` 实例中的数据的总结。每一个结构体都需要的行为是他们是能够被总结的，这样的话就可以调用实例的 `summarize` 方法来请求总结。示例 10-12 中展示了一个表现这个概念的 `Summary` trait 的定义：

文件名: src/lib.rs

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

其实就是接口

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

为了使用这个版本的 `Summary`，只需在实现 trait 时定义 `summarize_author` 即可：

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

#### [Trait Bound 语法](https://rustwiki.org/zh-CN/book/ch10-02-traits.html#trait-bound-语法)

`impl Trait` 语法适用于直观的例子，它实际上是一种较长形式语法的语法糖。我们称为 *trait bound*，它看起来像：

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```