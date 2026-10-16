// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

// 这个练习用到了一些我们课程后面才会讲的概念，比如 Box 和 From trait。现在不深入理解它们并不重要，但如果你想的话可以提前看看。目前，你可以把 Box<dyn ???> 这个类型理解为"我想要任何能 ??? 的东西"——考虑到 Rust 一贯的运行时安全标准，这个类型应该会让你觉得有点宽松！

// 简而言之，这种 Box 的特定用法，适用于这样的场景：你想拥有一个值，而你只关心它是实现了某个特定 trait 的类型。为此，Box 被声明为 Box<dyn Trait>，其中 Trait 就是编译器在那个上下文中会去查找的 trait。对于这个练习，那个上下文就是可能出现在 Result 里的错误类型。

// 我们能用什么来描述这两种错误呢？换句话说，有没有一个 trait，这两种错误都实现了？

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
// Result<(), E> 只能有一个 E。写 E = ParseIntError 装不下 CreationError；写 E = CreationError 装不下 ParseIntError
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;  // ① 可能返回 ParseIntError
    println!("output={:?}", PositiveNonzeroInteger::new(x)?); // ② 可能返回 CreationError
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
