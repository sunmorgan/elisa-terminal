use crate::models::Response;
use moka::future::Cache;
use std::time::Duration;

const CACHE_TTL_SECS: u64 = 30;

pub struct QuoteCache {
    cache: Cache<String, Response>,
}

impl QuoteCache {
    pub fn new() -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(CACHE_TTL_SECS))
                .build(),
        }
    }

    pub async fn get_or_fetch<F> {

    }
}
