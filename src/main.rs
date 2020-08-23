use anyhow::{anyhow, Result};
use fantoccini::{Client, Locator};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::env;
use std::time::Instant;

//TODO: Remove, testing purposes only
use dotenv::dotenv;
use std::time::Duration;
use tokio::time::delay_for;

mod newpipe_db;
use newpipe_db::NewpipeDB;
mod skiplist;
mod video;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let started = Instant::now();
    let mut c = Client::new("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    let newpipe_db = NewpipeDB::new()?;

    let bar = ProgressBar::new(newpipe_db.res.len() as u64);
    bar.set_style(ProgressStyle::default_bar().template(&format!(
        "{{wide_bar}}▏ {{pos}}/{{len}} {} ▏{{msg}}",
        HumanDuration(started.elapsed())
    )));

    sign_in(&mut c).await?;
    let mut skiplist = skiplist::Skiplist::load();

    for video in newpipe_db.res {
        let video = video;
        match like_video(&video.url, &mut c).await {
            Ok(()) => bar.set_message(&*format!("liked {} by {}", video.title, video.uploader)),
            Err(_e) => bar.set_message(&*format!("skipped {} by {}", video.title, video.uploader)),
        }
        skiplist.skiplist.insert(video.url);
        skiplist.save();

        bar.inc(1);
    }

    bar.finish_with_message("All done!");

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

    c.goto("https://accounts.google.com/signin/v2/identifier?service=youtube")
        .await?;

    let mut email_field = c.find(Locator::Id("identifierId")).await?;
    email_field.send_keys(&email).await?;

    let element = c.find(Locator::Id("identifierNext")).await?;
    element.click().await?;

    delay_for(Duration::from_millis(500)).await;

    let mut password_field = c
        .find(Locator::XPath(r#"//input[@name="password"]"#))
        .await?;
    password_field.send_keys(&password).await?;

    let element = c.find(Locator::Id("passwordNext")).await?;
    element.click().await?;

    Ok(())
}

async fn like_video(url: &str, c: &mut fantoccini::Client) -> Result<(), anyhow::Error> {
    c.goto(url).await?;

    let mut element = c
        .wait_for_find(Locator::XPath(
            r#"//button[contains(@aria-label, 'like this video')]"#,
        ))
        .await?;
    let result = element.attr("aria-pressed").await?;

    match result {
        Some(str) if str == "false" => {
            element.click().await?;
            Ok(())
        }
        _ => Err(anyhow!("Already liked")),
    }
}
