// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.
// From trait 用于“值到值”的转换。如果为一个类型正确地实现了 From，
// 那么 Into trait 应该能反向工作。你可以在此处阅读更多相关内容：
// https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results

// 步骤：
// 如果提供的字符串长度为 0，则返回 Person 的默认值。
// 以字符串中存在的逗号为分隔符，对字符串进行分割（Split）。
// 提取分割操作得到的第一个元素，将其作为 name（姓名）。
// 如果 name 为空，则返回 Person 的默认值。
// 提取分割操作得到的另一个元素，并将其解析（parse）为 usize 作为 age（年龄）。
// 如果在解析年龄的过程中出现任何错误，则返回 Person 的默认值；否则，返回一个用这些结果实例化的 Person 对象。

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }
        // 步骤 2：以逗号分割字符串
        let mut parts = s.split(',');
        // 步骤 3：提取第一个元素作为 name
        // 用 unwrap_or("") 防止没有逗号导致崩溃，用 trim() 去掉可能存在的空格
        let name = parts.next().unwrap_or("").trim();
        // 步骤 4：如果 name 为空，返回默认值
        if name.is_empty() {
            return Person::default();
        }
        // 步骤 5：取下一个元素解析为 age
        // 解析失败（无逗号 / 非数字 / 溢出 / 空串）都返回默认值
        // 如果 next() 得到 Some(s)，就对 s 调用 str::trim（即 s.trim()），得到去掉首尾空白的 &str 再包成 Some；如果 next() 是 None，结果还是 None。
        // map 的核心作用一句话：在不解包 Option（或 Result）的情况下，对里面的值做变换，保持容器结构不变。
        let age = match parts.next().map(str::trim).map(str::parse::<usize>) {
            Some(Ok(age)) => age,
            _ => return Person::default(),
        };
        // 关键补充：必须恰好两段，多出任何内容都算错误
        if parts.next().is_some() {
            return Person::default();
        }

        // 步骤 6：用结果实例化 Person
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
