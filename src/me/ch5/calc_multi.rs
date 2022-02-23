use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    let (tx, rx) = mpsc::channel();
    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num, answer)).unwrap();
        });
    }
    let mut job = request_nums.len();
    loop {
        if let Ok((arg, answer)) = rx.recv() {
            job -= 1;
            println!("[result] fib({}) = {} (remaining: {})", arg, answer, job);
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
        thread::sleep(Duration::from_millis(300));
    }
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!(
        "[time] {}.{:03} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}
