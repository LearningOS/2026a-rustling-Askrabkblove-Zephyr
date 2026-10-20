// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// “承接上一个练习，我们想要所有的线程都完成它们的工作，
// 但这一次，生成的子线程需要负责更新一个共享的值：JobStatus.jobs_completed。”
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 1. 用 Mutex 包起来，允许修改
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 2. 修改前，必须获取锁
            let mut status_guard = status_shared.lock().unwrap();
            // 3. 用 guard 去修改，而不是 status_shared
            status_guard.jobs_completed += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
        // 4. 打印最终值，需要再获取一次锁来读取
        let final_status = status.lock().unwrap();
        // 注意：这里打印 final_status.jobs_completed，而不是 handle
        println!("jobs completed {}", final_status.jobs_completed);
    }
}