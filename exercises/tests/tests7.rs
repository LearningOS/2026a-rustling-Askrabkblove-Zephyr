// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.
// 构建包时，有些依赖既不能在 Cargo.toml 中导入，也不能直接链接；有些预处理会因代码生成而异，用于设置包特定的配置。
//
// Cargo 并不打算取代其他构建工具，但它通过称为 build.rs 的自定义构建脚本与它们集成。这个文件通常放在项目根目录，而在本例中放在本练习的同一目录下。
//
// 它可以用于：
//
// - 构建捆绑的 C 库。
// - 在宿主系统上查找 C 库。
// - 根据规格说明生成 Rust 模块。
// - 执行 crate 所需的任何平台特定配置。
//
// 设置配置时，我们可以在构建脚本中用 println! 告诉 Cargo 遵循某些指令。通用格式是：
//
// println!("cargo:{}", your_command_in_string);
//
// 请参阅 Cargo 官方手册中关于构建脚本的内容以获取更多信息：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在本练习中，我们查找一个环境变量，并期望它落在某个范围内。你可以查看测试用例来了解细节。
//
// 你不应修改本文件。修改同一目录下的 build.rs 来通过本练习。
//
// 执行 rustlings hint tests7，或使用 hint 观察子命令来获取提示。


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
