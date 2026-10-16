// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.

// 一所想象中的魔法学校有一套用 Rust 编写的新的成绩单生成系统！目前，该系统只支持创建学生成绩以数字表示的成绩单（例如 1.0 -> 5.5）。然而，这所学校也会发放字母等级的成绩（A+ -> F-），并且需要能够打印这两种类型的成绩单！
// 请在 ReportCard 结构体和它的 impl 块中做出必要的代码修改，以支持字母等级的成绩单。将第二个测试中的 Grade 改为 "A+"，以证明你的修改允许使用字母等级。
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}
// ① impl —— 实现块的开始
// ② <T: std::fmt::Display> —— 声明泛型参数 T，并给它加一个约束（trait bound）：T 必须实现 std::fmt::Display
// ③ ReportCard<T> —— 为 ReportCard<T> 实现方法
impl<T: std::fmt::Display> ReportCard<T> {
//     pub fn notify<T: Summary>(item: T)
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
