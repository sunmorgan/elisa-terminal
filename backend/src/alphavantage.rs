use crate::{models:AvResponse};
use reqwest::Client;
use std::env;

const URL: &str = "https://www.alphavantage.co/query"
