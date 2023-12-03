use reqwest::Error;
use serde::de::StdError;
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

#[tokio::test]
async fn test_get_markets() {
    // get_markets 함수 호출
    match get_exchange_infos().await {
        Ok(info) => {
            // 필요한 테스트 조건을 추가하여 확인
            assert!(!info.symbols.is_empty());
        }
        Err(e) => {
            // 오류 발생 시 테스트 실패
            panic!("Test failed: {:?}", e);
        }
    }
}

pub async fn get_exchange_infos() -> Result<(ExchangeInfo), Error> {
    let request_url = format!("https://api.binance.com/api/v3/exchangeInfo");
    println!("url: {}", request_url);
    let response = reqwest::get(&request_url).await?;
    let info: ExchangeInfo = response.json().await?;
    println!("info: {:?}", info);
    Ok((info))
}
