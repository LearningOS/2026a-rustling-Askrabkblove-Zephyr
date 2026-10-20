// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// “这个程序会生成（spawn）多个线程，每个线程会运行至少 250 毫秒，
// 并且每个线程都会返回它们完成所花费的时间。
// 程序应该等待所有生成的线程完成，并将它们的返回值收集到一个向量（vector）中。
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // 计算从 start 时间点到现在所经过的时间，并将这个时间转换成“毫秒（milliseconds）”为单位的整数。
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        let time_taken = handle.join().unwrap();
        results.push(time_taken);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
