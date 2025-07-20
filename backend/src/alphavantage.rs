use crate::models::AvResponse;
use axum::response;
use reqwest::Client;
use std::env;

const URL: &str = "https://www.alphavantage.co/query";

pub struct AvClient {
    client: Client,
    api_key: String,
}

impl AvClient {
    pub fn new() -> Self {
        let api_key = env::var("api key").expect("missing key");

        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get_quote(&self, symbol: &str) -> Result<AvResponse> {
        let params = [
            ("function", "quote"),
            ("symbol", symbol),
            ("apikey", &self.api_key),
        ];

        let response = self
            .client
            .get(URL)
            .query(&params)
            .send()
            .await?
            .json::<AvResponse>()
            .await?;

        Ok(response)
    }
}
