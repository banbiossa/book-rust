use actix_web::{web, App, HttpRequest, HttpServer};

const SERVER_ADDR: &str = "127.0.0.1:8888";

// actix-web main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on {}", SERVER_ADDR);
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(SERVER_ADDR)?
        .run()
        .await
}

async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, World!"
}
