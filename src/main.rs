use fantoccini::{Client, Locator};
use std::env;
use anyhow::{Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle, HumanDuration};
use std::time::Instant;

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

mod newpipe_db {
    use rusqlite::{params, Connection};

    #[derive(Debug)]
    pub struct Video {
        id: i32,
        pub url: String,
        pub title: String,
        pub uploader: String
    }

    #[derive(Debug)]
    pub struct NewpipeDB {
        pub res: Vec<Video>,

    }

    pub async fn new_newpipe_db() -> Result<NewpipeDB, anyhow::Error> {
        let conn = Connection::open("./newpipe.db")?;
        let mut stmt = conn.prepare(
            "
            SELECT stream_id, streams.url, streams.title, streams.uploader 
            FROM stream_history 
            LEFT JOIN streams ON streams.uid=stream_history.stream_id 
            ORDER BY access_date DESC
            "
        )?;

        let res = stmt.query_map(params![], |row| {
            Ok(Video {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                uploader: row.get(3)?,
            })
        })?.map(|r| r.unwrap()).collect::<Vec<Video>>();

        let db = NewpipeDB {
            res: res,
        };

        Ok(db)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let started = Instant::now();
    let mut c = Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver");

    let newpipe_db = newpipe_db::new_newpipe_db().await?;

    let bar = ProgressBar::new(newpipe_db.res.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(&format!("{{wide_bar}}▏ {{pos}}/{{len}} {} ▏{{msg}}", HumanDuration(started.elapsed())))
        );

    sign_in(&mut c).await?;


    for video in newpipe_db.res {
        let video = video;
        match like_video(&video.url, &mut c).await {
            Ok(()) => bar.set_message(&*format!("liked {} by {}", video.title, video.uploader)),
            Err(_e) =>  bar.set_message(&*format!("skipped {} by {}", video.title, video.uploader)),

        }
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


    c.goto("https://accounts.google.com/signin/v2/identifier?service=youtube").await?;

    let mut email_field = c.find(Locator::Id("identifierId")).await?;
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