// use serde::{Deserialize, Serialize};
use tide::prelude::*;
const SERVER_ADDR: &str = "127.0.0.1:8080";

#[derive(Deserialize, Serialize)]
struct UserInfo {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Server started at {}", SERVER_ADDR);
    let mut app = tide::new();
    app.at("/").get(|_| async {
        Ok(tide::Response::builder(200)
            .content_type("text/html; charset=utf-8")
            .body(format!(
                "{}{}{}{}",
                "<html><body><form action='hello'>",
                "name: <input name='name' value='kujira'>",
                "<input type='submit' value='send'>",
                "</form></body></html>"
            ))
            .build())
    });
    app.at("/hello").get(|req: tide::Request<()>| async move {
        let user_info: UserInfo = req.query()?;
        Ok(tide::Response::builder(200)
            .content_type("text/html; charset=utf-8")
            .body(format!(
                "{}{}",
                "<h1>",
                format!("Hello {}!</h1>", user_info.name)
            ))
            .build())
    });
    app.listen(SERVER_ADDR).await?;
    Ok(())
}
