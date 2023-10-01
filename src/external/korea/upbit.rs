use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Code {
    market: String,
    korean_name: String,
    english_name: String,
    market_warning: Option<String>,
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


pub async fn get_code() -> Result<(), Error> {
    let request_url = format!("https://api.upbit.com/v1/market/all?isDetails=false");
    println!("url: {}", request_url);
    let response = reqwest::get(&request_url).await?;
    let codes: Vec<Code> = response.json().await?;
    codes.iter().for_each(|code| println!("{:?}", code));
    // println!("code: {:?}", codes);
    Ok(())
}

pub async fn get_ticker() -> Result<(), Error> {
    let request_url = format!("https://api.upbit.com/v1/ticker");
    reqwest::get(&request_url).await?;
    Ok(())
}