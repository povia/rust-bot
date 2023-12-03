use reqwest::Error;
use serde::Deserialize;
use crate::external::binance::spot::get_exchange_infos;

const UPBIT_URL: &str = "https://api.upbit.com";

#[derive(Deserialize, Debug)]
pub struct Code {
    market: String,
    korean_name: String,
    english_name: String,
    market_warning: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MarketData {
    market: String,
    trade_date: String,
    trade_time: String,
    trade_date_kst: String,
    trade_time_kst: String,
    trade_timestamp: i64,
    opening_price: f64, // f64로 변경
    high_price: f64, // f64로 변경
    low_price: f64, // f64로 변경
    trade_price: f64, // f64로 변경
    prev_closing_price: f64, // f64로 변경
    change: String,
    change_price: f64, // f64로 변경
    change_rate: f64, // f64로 변경
    signed_change_price: f64, // f64로 변경
    signed_change_rate: f64, // f64로 변경
    trade_volume: f64, // f64로 변경
    acc_trade_price: f64, // f64로 변경
    acc_trade_price_24h: f64, // f64로 변경
    acc_trade_volume: f64, // f64로 변경
    acc_trade_volume_24h: f64, // f64로 변경
    highest_52_week_price: f64, // f64로 변경
    highest_52_week_date: String,
    lowest_52_week_price: f64, // f64로 변경
    lowest_52_week_date: String,
    timestamp: i64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBook {
    market: String,
    // 	마켓 코드
    timestamp: i64,
    // 	호가 생성 시각
    total_ask_size: String,
    // 호가 매도 총 잔량
    total_bid_size: String,
    // 	호가 매수 총 잔량
    orderbook_units: Vec<OrderBookUnit>, // 호가
}

#[derive(Deserialize, Debug)]
pub struct OrderBookUnit {
    ask_price: String,
    // 매도 호가
    bid_price: String,
    // 	매수호가
    ask_size: String,
    // 	매도 잔량
    bid_size: String, // 매수 잔량
}


#[tokio::test]
async fn test_get_code() {
    // get_markets 함수 호출
    match get_code().await {
        Ok(codes) => {
            // 필요한 테스트 조건을 추가하여 확인
            assert!(!codes.is_empty());
        }
        Err(e) => {
            // 오류 발생 시 테스트 실패
            panic!("Test failed: {:?}", e);
        }
    }
}


#[tokio::test]
async fn test_get_ticker() {
    // 특정 market 값을 전달하여 get_ticker 함수 호출
    let market = "KRW-BTC"; // 테스트하고자 하는 market 값
    match get_ticker(market.to_string()).await {
        Ok(tickers) => {
            println!("Ticker for {} retrieved successfully.", market);
            assert!(!tickers.is_empty());
            println!("Tickers : {:?}", tickers);
        }
        Err(e) => {
            // 오류 발생 시 테스트 실패
            panic!("Test failed for {}: {:?}", market, e);
        }
    }
}

pub async fn get_code() -> Result<(Vec<Code>), Error> {
    let request_url = format!("{}/v1/market/all?isDetails=false", UPBIT_URL);
    println!("url: {}", request_url);
    let response = reqwest::get(&request_url).await?;
    let codes: Vec<Code> = response.json().await?;
    codes.iter().for_each(|code| println!("{:?}", code));
    Ok((codes))
}

pub async fn get_ticker(code: String) -> Result<(Vec<MarketData>), Error> {
    let request_url =format!("{}/v1/ticker?markets={}", UPBIT_URL, code);
    let response = reqwest::get(&request_url).await?;
    let tickers: Vec<MarketData> = response.json().await?;
    Ok((tickers))
}