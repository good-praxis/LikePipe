use fantoccini::{Client, Locator};
use dotenv::dotenv;
use std::env;

//TODO: Remove, testing purposes only
use tokio::time::delay_for; 
use std::time::Duration;

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to youtube login page
    c.goto("https://accounts.google.com/signin/v2/identifier?service=youtube").await?;

    delay_for(Duration::from_millis(5000)).await;

    // access the identifier field
    // c.find(Locator::Id("IdentifierId"))


    c.close().await
}