use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RateLimit {
    rateLimitType: String,
    interval: String,
    limit: i32,
}

#[derive(Debug, Deserialize)]
pub struct Symbol {
    symbol: String,
    status: String,
    baseAsset: String,
    quoteAsset: String,
    baseAssetPrecision: i32,
    quoteAssetPrecision: i32,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfo {
    timezone: String,
    serverTime: i64,
    rateLimits: Vec<RateLimit>,
    exchangeFilters: Vec<serde_json::Value>, // 무시할 필요가 있는 경우 추가
    symbols: Vec<Symbol>,
}


pub async fn getMarkets() -> Result<(ExchangeInfo), Error> {
    let request_url = format!("https://api.binance.com/api/v3/exchangeInfo");
    println!("url: {}", request_url);
    let response = reqwest::get(&request_url).await?;
    let info: ExchangeInfo = response.json().await?;
    println!("info: {:?}", info);
    Ok((info))
}