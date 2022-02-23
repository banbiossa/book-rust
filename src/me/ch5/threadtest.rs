use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{} i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    println!("Not threaad");
    sleep_print("Not threaad");

    println!("\nThread");
    sleep_print("Thread");

    thread::spawn(|| {
        sleep_print("Jiro");
    });
    thread::spawn(|| {
        sleep_print("Siro");
    });
    sleep_print("Taro");
}
