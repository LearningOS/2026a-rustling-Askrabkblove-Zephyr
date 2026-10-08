切片：

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

其实和python切片差不多，都是【）闭区间到开区间，省略的都是首尾

```rust
fn first_word(s: &String) -> &str {
    // as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();
	// 使用 iter 方法在字节数组上创建一个迭代器
    // enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
	// 和 &s[0..s.len()]
    &s[..]
}

fn main() {}

```

