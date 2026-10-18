

```rust
fn largest(list: &[i32]) -> i32 { 
    //  [i32] —— 切片类型（slice），表示"一串 i32"，但长度未知
	//	&[i32] —— 对切片的引用，即"借来看的一串 i32"
	//	读作："一个指向 i32 切片的引用"。
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

