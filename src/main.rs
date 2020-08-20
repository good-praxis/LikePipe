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

    sign_in(&mut c).await?;

    // Liking test
    let url = "https://www.youtube.com/watch?v=DLzxrzFCyOs";
    like_video(&url, &mut c).await?;
    
    delay_for(Duration::from_millis(5000)).await;


    c.close().await
}

async fn sign_in(c: &mut fantoccini::Client) -> Result<(), fantoccini::error::CmdError> {
    dotenv().ok();

    let email: String = match env::var("YOUTUBE_EMAIL") {
        Ok(val) => val,
        Err(e) => panic!("error parsing YOUTUBE_EMAIL: {}", e),
    };

    let password: String = match env::var("YOUTUBE_PASSWORD") {
        Ok(val) => val,
        Err(e) => panic!("error parsing YOUTUBE_PASSWORD: {}", e),
    };


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

    Ok(())
}

async fn like_video(url: &str, c: &mut fantoccini::Client) -> Result<(), fantoccini::error::CmdError> {
    c.goto(url).await?;

    let mut element = c.wait_for_find(Locator::XPath(r#"//button[contains(@aria-label, 'like this video')]"#)).await?;
    let result = element.attr("aria-pressed").await?;

    match result {
        Some(str) if str == "false" => {
            element.click().await?;
            println!("Liked!");   
        },
        _ => println!("Moving on"),
    }

    Ok(())
}