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