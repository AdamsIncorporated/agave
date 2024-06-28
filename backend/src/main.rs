use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use tokio_test;
use yahoo_finance_api as yahoo;

#[derive(Serialize)]
struct Quote {
    time: String,
    close: f64,
}

async fn get_quote() -> Quote {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1d")).unwrap();
    let quote = response.last_quote().unwrap();
    let time: String =
        OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp)).to_string();

    Quote {
        time: time,
        close: quote.close,
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let quote = get_quote().await;
    let json = serde_json::to_string(&quote).unwrap_or_default();

    HttpResponse::Ok().body(json)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
