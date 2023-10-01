mod external;

use reqwest::Error;
use crate::external::korea::upbit;

#[tokio::main]
async fn main() -> Result<(), Error> {
    upbit::get_code().await?;
    Ok(())
}

