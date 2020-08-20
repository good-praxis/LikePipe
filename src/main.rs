use fantoccini::{Client, Locator};
use dotenv::dotenv;
use std::env;

//TODO: Remove, testing purposes only
use tokio::time::delay_for; 
use std::time::Duration;

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    dotenv().ok();

    let email: String = match env::var("YOUTUBE_EMAIL") {
        Ok(val) => val,
        Err(e) => panic!("error parsing YOUTUBE_EMAIL: {}", e),
    };

    let password: String = match env::var("YOUTUBE_PASSWORD") {
        Ok(val) => val,
        Err(e) => panic!("error parsing YOUTUBE_PASSWORD: {}", e),
    };


    let mut c = Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // first, go to youtube login page
    c.goto("https://accounts.google.com/signin/v2/identifier?service=youtube").await?;

    // access the identifier field
    let mut email_field = c.find(Locator::Id("identifierId")).await?;

    // enter email into field
    email_field.send_keys(&email).await?;

    let element = c.find(Locator::Id("identifierNext")).await?;
    element.click().await?;

    delay_for(Duration::from_millis(500)).await;

    let mut password_field = c.find(Locator::XPath(r#"//input[@name="password"]"#)).await?;

    password_field.send_keys(&password).await?;

    let element = c.find(Locator::Id("passwordNext")).await?;
    element.click().await?;


    delay_for(Duration::from_millis(5000)).await;


    c.close().await
}