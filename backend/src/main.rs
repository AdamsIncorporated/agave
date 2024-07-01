use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;
use serde::Serialize;
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::YahooError;
mod cors;

#[derive(Serialize)]
struct YahooApiData {
    time: String,
    close: f64,
    open: f64,
    high: f64,
    low: f64,
    volume: u64,
    adjclose: f64,
}

async fn get_quote() -> Result<YahooApiData, YahooError> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes("AAPL", "1d").await?;
    let quote = response.last_quote().unwrap();
    let time: String =
        OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp)).to_string();

    Ok(YahooApiData {
        time: time,
        close: quote.close,
        open: quote.open,
        high: quote.high,
        low: quote.low,
        volume: quote.volume,
        adjclose: quote.adjclose,
    })
}

#[get("/")]
async fn index() -> impl Responder {
    match get_quote().await {
        Ok(quote) => {
            let json = serde_json::to_string(&quote).unwrap_or_else(|err| {
                info!("{}", err.to_string());
                err.to_string()
            });
            info!("Response JSON: {}", json);
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json)
        }
        Err(err) => {
            info!("{}", err.to_string());
            HttpResponse::InternalServerError().body(format!("Failed to get quote: {}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = cors::create_cors();
        App::new()
            .wrap(cors)
            .service(index)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
