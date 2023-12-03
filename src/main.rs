mod external;

use reqwest::Error;
use crate::external::korea::upbit;
use crate::external::binance::spot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // upbit::get_code().await?;
    spot::get_exchange_infos().await?;
    Ok(())
}

