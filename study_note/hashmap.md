可以使用 `new` 创建一个空的 `HashMap`，并使用 `insert` 增加元素。

```rust

#![allow(unused)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
}

use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // iter迭代器
```

