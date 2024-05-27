mod services {
    pub mod yahoo_finance;
}
mod postgres;
mod user;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use deadpool_postgres::Pool;
use qstring::QString;
use services::yahoo_finance::AuthConfig;
use std::result::Result::*;
use std::string::String;

#[get("/users")]
async fn list_users(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match user::User::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

#[get("/fetch-yahoo-data")]
async fn fetch_yahoo_data(req: HttpRequest) -> HttpResponse {
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let code = qs.get("code").unwrap();

    HttpResponse::Ok().json(code)
}

#[get("/yahoo-send-to-consent")]
async fn yahoo_send_to_consent(req: HttpRequest) -> HttpResponse {
    let config = AuthConfig::new();

    match config.request_auth().await {
        Ok(_) => println!("Auth request successful"),
        Err(e) => eprintln!("Auth request error: {}", e),
    }

    // After getting the authorization code from the redirect URL, call get_token
    let auth_code = "authorization_code_from_redirect";
    match config.get_token(auth_code).await {
        Ok(token_response) => println!("Token obtained: {:?}", token_response),
        Err(e) => eprintln!("Token request error: {}", e),
    }

    HttpResponse::Ok().json("200")
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();
    postgres::migrate_up(&pg_pool).await;

    let address = address();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(list_users)
            .service(fetch_yahoo_data)
            .service(yahoo_send_to_consent)
    })
    .bind(&address)?
    .run()
    .await
}
