// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y { // 引用不会移交所有权
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y), // 所有权移交给Some(p)
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
