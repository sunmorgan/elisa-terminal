use axum::{routing::get, Router};

mod alphavantage;
mod cache;
mod models;

use axum::{extract::Path, routing::get, Router};

use cache::QuoteCache;
use models::Response;
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber;

struct AppState {
    alpha_client: alphavantage::AvClient,
    quote_cache: cache::QuoteCache,
}

async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        alpha_client: alphavantage::AvClient::new(),
        quote_cache: cache::QuoteCache::new(),
    });
}
