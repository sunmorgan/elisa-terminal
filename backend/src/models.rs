use serde::Deserialize;

pub struct Quote {
    pub symbol: String,
    pub price: String,
    pub change: String,
    pub change_percent: String,
}

pub struct AvResponse {
    pub quote: Option<Quote>,
    pub error: Option<String>,
}

pub struct response {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: String,
}
