use anyhow::Result;
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
mod webdriver;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let started = Instant::now();

    let (email, secret) = get_credentials();

    let mut c = webdriver::Client::new(&email, &secret).await?;

    let newpipe_db = NewpipeDB::new()?;

    let bar = ProgressBar::new(newpipe_db.res.len() as u64);
    bar.set_style(ProgressStyle::default_bar().template(&format!(
        "{{wide_bar}}▏ {{pos}}/{{len}} {} ▏{{msg}}",
        HumanDuration(started.elapsed())
    )));

    let mut skiplist = skiplist::Skiplist::load();

    for video in newpipe_db.res {
        let video = video;
        match c.like_video(&video.url).await {
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

fn get_credentials() -> (String, String) {
    dotenv().ok();

    let email = env::var("YOUTUBE_EMAIL").expect(".env parameter 'YOUTUBE_EMAIL' not set");
    let secret = env::var("YOUTUBE_SECRET").expect(".env parameter 'YOUTUBE_SECRET' not set");

    (email, secret)
}
