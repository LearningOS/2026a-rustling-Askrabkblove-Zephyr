### [使用 `spawn` 创建新线程](https://rustwiki.org/zh-CN/book/ch16-01-threads.html#使用-spawn-创建新线程)

为了创建一个新线程，需要调用 `thread::spawn` 函数并传递一个闭包（第 13 章学习了闭包），并在其中包含希望在新线程运行的代码。示例 16-1 中的例子在主线程打印了一些文本而另一些文本则由新线程打印：

文件名: src/main.rs

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

#### [使用 `join` 等待所有线程结束](https://rustwiki.org/zh-CN/book/ch16-01-threads.html#使用-join-等待所有线程结束)

由于主线程结束，示例 16-1 中的代码大部分时候不光会提早结束新建线程，甚至不能实际保证新建线程会被执行。其原因在于无法保证线程运行的顺序！

可以通过将 `thread::spawn` 的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。`thread::spawn` 的返回值类型是 `JoinHandle`。`JoinHandle` 是一个拥有所有权的值，当对其调用 `join` 方法时，它会等待其线程结束。示例 16-2 展示了如何使用示例 16-1 中创建的线程的 `JoinHandle` 并调用 `join` 来确保新建线程在 `main` 退出前结束运行：

文件名: src/main.rs

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

`mpsc::channel` 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。由于历史原因，`tx` 和 `rx` 通常作为 **发送者**（*transmitter*）和 **接收者**（*receiver*）的缩写，所以这就是我们将用来绑定这两端变量的名字。这里使用了一个 `let` 语句和模式来解构了此元组；第 18 章会讨论 `let` 语句中的模式和解构。如此使用 `let` 语句是一个方便提取 `mpsc::channel` 返回的元组中一部分的手段。

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

```

### [通过克隆发送者来创建多个生产者](https://rustwiki.org/zh-CN/book/ch16-02-message-passing.html#通过克隆发送者来创建多个生产者)

之前我们提到了`mpsc`是 *multiple producer, single consumer* 的缩写。可以运用 `mpsc` 来扩展示例 16-10 中的代码来创建向同一接收者发送值的多个线程。这可以通过克隆通道的发送端来做到，如示例 16-11 所示：

文件名: src/main.rs

```rust
// --snip--

let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
```

示例 16-11: 从多个生产者发送多个消息

这一次，在创建新线程之前，我们对通道的发送端调用了 `clone` 方法。这会给我们一个可以传递给第一个新建线程的发送端句柄。我们会将原始的通道发送端传递给第二个新建线程。这样就会有两个线程，每个线程将向通道的接收端发送不同的消息。