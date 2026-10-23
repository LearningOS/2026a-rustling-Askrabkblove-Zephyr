- `TcpListener::bind("地址:端口")` —— 把程序**绑定**到 `127.0.0.1:7878` 这个地址上，开始监听。
  - `127.0.0.1` 是**本机回环地址**，只有本机能连，外部机器连不到。
  - `7878` 是端口号，随便挑的。
- `bind` 返回 `Result<TcpListener>`：
  - 成功 → `Ok(listener)`
  - 失败（比如端口被占用、没权限）→ `Err(...)`
- `.unwrap()` —— 成功就取出 `listener`，失败就 **panic 崩溃**。教程这里图省事，真实项目应处理错误。

```rust
use std::net::TcpListener;// 从标准库的网络模块引入 TcpListener。它是服务端用来监听端口、接受连接的类型。

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {// listener.incoming() 返回一个迭代器，每次迭代产出一个"尝试接受的连接"。
        let stream = stream.unwrap(); // .unwrap() 取出真正的 TcpStream——它代表这条已建立的 TCP 连接，可以用来读写数据。

        println!("Connection established!");
    }
}
```

### [读取请求](https://rustwiki.org/zh-CN/book/ch20-01-single-threaded.html#读取请求)

让我们实现读取来自浏览器请求的功能！为了分离获取连接和接下来对连接的操作的相关内容，我们将开始一个新函数来处理连接。在这个新的 `handle_connection` 函数中，我们从 TCP 流中读取数据并打印出来以便观察浏览器发送过来的数据。将代码修改为如示例 20-2 所示：

文件名: src/main.rs

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {// 这里的 mut 修饰的是变量绑定 stream，意思是：这个局部变量本身允许被可变借用，也允许被重新赋值。
    let mut buffer = [0; 1024];
	// fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```