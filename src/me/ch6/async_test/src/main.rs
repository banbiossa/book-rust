use tokio;

#[tokio::main]
async fn main() {
    let f = say_later("Give up, you're not a Rustacean!");

    println!("I might be a pythonista");

    f.await;
}

async fn say_later(msg: &'static str) {
    println!("{}", msg);
}
