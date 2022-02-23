use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

const SERVER_ADDR: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on {}", SERVER_ADDR);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/calc", web::get().to(calc))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "{}{}{}{}{}{}",
            "<html><body><h1>BMI Calculator</h1>",
            "<form action='calc'>",
            "height: <input name='height' value='160'><br>",
            "weight: <input name='weight' value='60'><br>",
            "<input type='submit' value='Calculate'></form>",
            "</form></body></html>"
        )))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FormBMI {
    height: f64,
    weight: f64,
}

async fn calc(q: web::Query<FormBMI>) -> Result<HttpResponse, Error> {
    println!("{:?}", q);
    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h3>BMI={:.1}, PER={:.0}%</h3>", bmi, per)))
}
