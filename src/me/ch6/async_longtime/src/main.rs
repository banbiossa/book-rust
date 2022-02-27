use tokio::time;

#[tokio::main]
async fn main() {
    for i in 1..=3 {
        println!("{}", i);
        let s = read_longtime().await;

        println!("{}", s);
        let s = async {
            time::sleep(time::Duration::from_secs(1)).await;
            String::from("[block] long wait has ended.")
        }
        .await;
        println!("{}", s);
    }
}

async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("[function] long wait has ended.")
}
