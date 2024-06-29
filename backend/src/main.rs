use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;
use serde::Serialize;
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::YahooError;

#[derive(Serialize)]
struct YahooApiData {
    time: String,
    close: f64,
}

async fn get_quote() -> Result<YahooApiData, YahooError> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes("AAPL", "1d").await?;
    let quote = response.last_quote().unwrap(); // Note: Handle potential errors here
    let time: String =
        OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp)).to_string();

    Ok(YahooApiData {
        time: time,
        close: quote.close,
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
            HttpResponse::Ok().body(json)
        },
        Err(err) => {
            info!("{}", err.to_string());
            HttpResponse::InternalServerError().body(format!("Failed to get quote: {}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
