### [结合 `Rc` 和 `RefCell` 来拥有多个可变数据所有者](https://rustwiki.org/zh-CN/book/ch15-05-interior-mutability.html#结合-rct-和-refcellt-来拥有多个可变数据所有者)

Rust 有个铁律：

> 你有一个**不可变引用** `&T` 时，**不能**通过它修改数据。

比如：

rust

```
let x = 5;
let y = &mut x;   // ❌ 编译错误:x 不是 mut,不能可变借用
```



**但有时候你就是想「外面看起来不可变，里面偷偷能改」**。这叫**内部可变性**（interior mutability）。

为什么想要这个？看下面的真实需求。

## 二、真实需求：写一个「假的」发消息器来测试

书里在写一个「限额追踪器」`LimitTracker`：

- 它记录你用了多少额度（`value`）、上限多少（`max`）
- 用量超过 75% / 90% / 100% 时，调用 `messenger.send(...)` 发警告

`Messenger` 是个 trait（接口），只有一个方法：

rust

```
pub trait Messenger {
    fn send(&self, msg: &str);   // 注意:&self,不可变借用!
}
```



**关键点：`send` 接收的是 `&self`（不可变引用）。** 这是 trait 定死的，实现者不能改。

现在要**测试** `LimitTracker`。测试时我们不想真发短信、真发邮件，而是想造一个「假 messenger」，它**不真发，只把消息记下来**，最后检查「记了几条、内容对不对」。

于是你写：

rust

```
struct MockMessenger {
    sent_messages: Vec<String>,   // 把发过的消息记在这里
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.push(String::from(message));  // ❌ 编译错误!
    }
}
```



**报错**：`send` 拿的是 `&self`（不可变），但你却想 `push`（修改 `sent_messages`）。不可变引用不能改数据，编译不过。

你可能会想：那把 `send` 改成 `&mut self` 不就行了？**不行**——trait 里定义的是 `&self`，实现必须一致，改了就不符合 `Messenger` 了。

**矛盾就在这里**：

- trait 要求 `send(&self)` —— 不可变
- 但你想在 `send` 里改 `sent_messages` —— 需要可变

## 三、办法：用 `RefCell` 把「检查」从编译期挪到运行期

`RefCell<T>` 是一个**容器**，它的魔法是：

> **即使你只有它的不可变引用，也能修改它里面的值。**

把 `sent_messages` 从 `Vec<String>` 换成 **`RefCell<Vec<String>>`**：

rust

```
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,   // 关键:套一层 RefCell
}
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // borrow_mut() 拿到内部 Vec 的可变引用,即使 self 是不可变的!
        self.sent_messages.borrow_mut().push(String::from(message));   // ✅ 能编译了
    }
}
```

- `self` 还是 `&self`（不可变），符合 trait
- 但 `self.sent_messages.borrow_mut()` 能拿到内部 `Vec` 的**可变**引用
- 于是 `push` 成功

这就是**内部可变性**：外面（`self`）看起来不可变，里面（`RefCell` 包的值）却能改。

## 四、`RefCell` 怎么做到的？它把借用检查推迟到运行时

普通引用 `&` / `&mut`：**编译时**检查借用规则。违反了 → 编译错误。

`RefCell`：**运行时**检查。它内部记着「现在有几个不可变借用、几个可变借用」：

| 方法           | 作用                       | 相当于 |
| :------------- | :------------------------- | :----- |
| `borrow()`     | 借出内部值的**不可变**引用 | `&`    |
| `borrow_mut()` | 借出内部值的**可变**引用   | `&mut` |

规则还是老的（多个不可变 **或** 一个可变，不能同时），但**违反时不报编译错，而是运行时 panic**。

书里故意演示了违规：

rust

```
fn send(&self, message: &str) {
    let mut one = self.sent_messages.borrow_mut();   // 第一个可变借用
    let mut two = self.sent_messages.borrow_mut();   // ❌ 同一作用域第二个可变借用
    ...
}
```



编译**能过**，但一运行就 panic：

text

```
already borrowed: BorrowMutError
```



因为 `RefCell` 在运行时发现「你已经有可变借用了，又借一个，违规！」，直接崩溃。

## 五、`Rc<RefCell<T>>`：多个所有者 + 能改

书最后讲一个组合技。先回顾两个类型各自的能力：

| 类型         | 能干什么                     | 缺点                         |
| :----------- | :--------------------------- | :--------------------------- |
| `Rc<T>`      | **多个所有者**共享同一份数据 | 只能**读**，不能改（不可变） |
| `RefCell<T>` | 外面不可变也能**改**内部     | 只有一个所有者               |

把它们套起来 **`Rc<RefCell<T>>`**：

- `Rc` 负责「**多人共享**」
- `RefCell` 负责「**共享的同时还能改**」

于是你得到了「**多个所有者 + 可修改**」的数据。书里用 cons list 演示：

rust

```
let value = Rc::new(RefCell::new(5));

let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
let b = Cons(..., Rc::clone(&a));
let c = Cons(..., Rc::clone(&a));

*value.borrow_mut() += 10;   // 通过 value 改内部值
```



因为 `a`、`b`、`c` 都共享 `value`（`Rc`），改 `value` 里的 5 为 15 后，**打印 a、b、c 都会看到 15**——大家都指向同一份数据，且这份数据被改了。

text

```
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
                                              ↑ 共享的那个 15
```

## 六、一句话总结整节

> **正常 Rust：不可变引用不能改数据（编译时检查）。**
> **但有时（如测试里的 mock）你需要在「不可变」的方法里改东西。**
> **`RefCell<T>` 让你能改，代价是把借用检查从编译时挪到运行时——违规不报编译错，而是运行 panic。**
> **`Rc<RefCell<T>>` 再进一步：多人共享 + 都能改。**

## 七、对照原文的关键句（帮你回读）

| 原文                                     | 人话                                                 |
| :--------------------------------------- | :--------------------------------------------------- |
| 内部可变性允许你在有不可变引用时改变数据 | 外面 `&T`，里面照样能改                              |
| `RefCell<T>` 的借用规则作用于运行时      | 编译不管，运行才查，违规 panic                       |
| 编译器是保守的，可能拒绝正确程序         | 编译器有时看不懂你的代码是对的，`RefCell` 给你绕过它 |
| `borrow` / `borrow_mut`                  | 相当于运行时的 `&` / `&mut`                          |
| `Rc<RefCell<T>>` 多所有者且可修改        | 共享 + 可变，两个能力叠加                            |

## 八、你只需要记住的三件事

1. **`RefCell<T>` 解决什么**：想在「只有 `&self`」的地方改数据（编译期不让，运行期放行）。
2. **怎么用**：`borrow()` 读、`borrow_mut()` 写；违规会 **panic**，不是编译错。
3. **为什么 `Rc<RefCell<T>>`**：`Rc` 给「多人共享」，`RefCell` 给「共享还能改」，合起来就是「多个所有者都能改同一份数据」。

看不懂原文是正常的——它把「为什么需要」和「怎么实现」混在一起讲。你按上面「矛盾 → 办法 → 例子」的顺序再回读，应该就顺了。要不要我再用一个**最小可运行例子**（不用书里的 mock）演示 `RefCell` 的读写和 panic？