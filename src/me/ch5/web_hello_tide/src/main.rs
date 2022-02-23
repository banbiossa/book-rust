const SERVER_ADDR: &str = "127.0.0.1:8080";

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Server listening on {}", SERVER_ADDR);
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen(SERVER_ADDR).await?;
    Ok(())
}
