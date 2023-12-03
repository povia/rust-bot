mod external;

use reqwest::Error;
use crate::external::korea::upbit;
use crate::external::binance::spot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // upbit::get_code().await?;
    let binanceInfo = spot::getMarkets().await?;
    Ok(())
}

