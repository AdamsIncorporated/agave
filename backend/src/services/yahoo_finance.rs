use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    client_id: String,
    client_secret: String,
    redirect_url: String,
    auth_endpoint: String,
    token_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    token_type: String,
}

impl AuthConfig {
    pub fn new() -> Self {
        let client_id = env::var("CLIENT_ID")
            .expect("CLIENT_ID not set")
            .to_string();
        let client_secret = env::var("CLIENT_SECRET")
            .expect("CLIENT_SECRET not set")
            .to_string();
        let redirect_url = env::var("REDIRECT_URL")
            .expect("REDIRECT_URL not set")
            .to_string();

        AuthConfig {
            client_id: client_id,
            client_secret: client_secret,
            redirect_url: redirect_url,
            auth_endpoint: "https://api.login.yahoo.com".to_string(),
            token_endpoint: "https://api.login.yahoo.com/oauth2/get_token".to_string(),
        }
    }

    pub async fn request_auth(&self) -> Result<(), Box<dyn Error>> {
        let params = [
            ("client_id", &self.client_id.as_str()),
            ("redirect_uri", &self.redirect_url.as_str()),
            ("response_type", &"code"),
        ];

        let client = Client::new();
        let res = client
            .get(&self.auth_endpoint)
            .query(&params)
            .send()
            .await?;

        if res.status().is_success() {
            println!("{:#?}", res.text().await?);
            Ok(())
        } else {
            Err(format!("Failed to request auth: {:?}", res.status()).into())
        }
    }

    pub async fn get_token(&self, code: &str) -> Result<TokenResponse, Box<dyn Error>> {
        let params = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_url),
            ("code", &code.to_string()),
            ("grant_type", &"authorization_code".to_string()),
        ];

        let client = Client::new();
        let res = client
            .post(&self.token_endpoint)
            .form(&params)
            .send()
            .await?;

        if res.status().is_success() {
            let token_response: TokenResponse = res.json().await?;
            println!("{:#?}", token_response);
            Ok(token_response)
        } else {
            Err(format!("Failed to get token: {:?}", res.status()).into())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StockQuote {
    symbol: String,
    regular_market_price: f64,
    regular_market_change: f64,
    regular_market_change_percent: f64,
}
