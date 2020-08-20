use fantoccini::{Client, Locator};
use rusqlite::{params, Connection};
use std::env;
use anyhow::{Result, anyhow};

//TODO: Remove, testing purposes only
use tokio::time::delay_for; 
use std::time::Duration;
use dotenv::dotenv;

#[derive(Debug)]
struct Video {
    id: i32,
    url: String,
    title: String,
    uploader: String
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut c = Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver");

    let conn = Connection::open("./newpipe.db")?; 
    let mut stmt = conn.prepare(
        "
        SELECT stream_id, streams.url, streams.title, streams.uploader 
        FROM stream_history 
        LEFT JOIN streams ON streams.uid=stream_history.stream_id 
        ORDER BY access_date DESC
        "
    )?;
    let total_count = stmt.query_map(params![], |_row| {
        Ok(())
    })?.count();
    let video_iter = stmt.query_map(params![], |row| {
        Ok(Video {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            uploader: row.get(3)?,
        })
    })?;

    sign_in(&mut c).await?;

    for video in video_iter.enumerate() {
        let (index, video) = video;
        let video = video.unwrap();
        match like_video(&video.url, &mut c).await {
            Ok(()) => println!("Step {}/{}, liked {} by {}", index+1, total_count, video.title, video.uploader),
            Err(_e) =>  println!("Step {}/{}, skipping {} by {}", index+1, total_count, video.title, video.uploader),

        }
    }

    delay_for(Duration::from_millis(5000)).await;


    Ok(c.close().await?)
}

async fn sign_in(c: &mut fantoccini::Client) -> Result<(), anyhow::Error> {
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

async fn like_video(url: &str, c: &mut fantoccini::Client) -> Result<(), anyhow::Error> {
    c.goto(url).await?;

    let mut element = c.wait_for_find(Locator::XPath(r#"//button[contains(@aria-label, 'like this video')]"#)).await?;
    let result = element.attr("aria-pressed").await?;

    match result {
        Some(str) if str == "false" => {
            element.click().await?;
            Ok(())   
        },
        _ => Err(anyhow!("Already liked")),
    }

}