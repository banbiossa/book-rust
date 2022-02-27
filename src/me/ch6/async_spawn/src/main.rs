use tokio::time;

async fn say_later(sec: u64, msg: &str) {
    time::sleep(time::Duration::from_secs(sec)).await;
    println!("{}: {}", sec, msg);
}

#[tokio::main]
async fn main() {
    tokio::spawn(say_later(1, "hello"));
    tokio::spawn(say_later(2, "world"));
    tokio::spawn(say_later(3, "!"));
    time::sleep(time::Duration::from_secs(5)).await;
    println!("done");

    tokio::join!(
        say_later(2, "I work very hard to make this work"),
        say_later(1, "there is nothing more fun than our lives"),
        say_later(3, "eat, drink and be merry"),
    );
}
