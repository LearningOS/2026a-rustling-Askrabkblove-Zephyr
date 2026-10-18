// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect(), // 本来想用闭包，结果String::from不支持
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
// 遍历每个&str，调用capitalize_first，收集到Vec<String>
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut vec = vec![];
    // words: &[&str]，所以 words.iter() 迭代出的是 &&str。
    for ss in words.iter() {
        vec.push(capitalize_first(*ss));
    }
    vec
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
// 遍历每个&str，调用capitalize_first，收集到Vec<String>
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut sstr = String::new();
    for ss in words.iter() {
        sstr += &capitalize_first(*ss);
    }
    sstr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
