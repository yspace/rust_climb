use thirtyfour::{error::WebDriverError, prelude::*};
use tokio;

use web_scraper2::* ;

#[tokio::main]
async fn main() -> Result<(), WebDriverError> {
    // return examples::foo1().await;
    // return examples::scrape_wikipedia().await ;
    // return examples::scrape2().await ;
    return save2sqlite::run().await ;
}