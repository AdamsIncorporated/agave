mod ws;

use actix_web::{web, App, Error, HttpResponse, HttpServer};

async fn index() -> Result<HttpResponse, Error> {
    let resp = HttpResponse::Ok().body("Hello World!");
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
