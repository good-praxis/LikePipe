use anyhow::Result;
use fantoccini::Locator;

use std::process::Command;
use std::time::Duration;
use tokio::time::delay_for;

mod error;
use error::Error;

pub struct Client {
    c: fantoccini::Client,
}

impl Client {
    pub async fn new(email: &str, secret: &str) -> Result<Client, error::Error> {
        Command::new("geckodriver")
            .spawn()
            .expect("geckodriver start failed");
        let c = fantoccini::Client::new("http://localhost:4444").await;
        let mut c = match c {
            Ok(c) => Client { c: c },
            Err(_) => return Err(Error::WebdriverNotFound),
        };

        match c.sign_in(email, secret).await {
            Err(_) => Err(Error::SignInFailed),
            Ok(_) => Ok(c),
        }
    }

    async fn sign_in(&mut self, email: &str, secret: &str) -> Result<(), anyhow::Error> {
        self.c
            .goto("https://accounts.google.com/signin/v2/identifier?service=youtube")
            .await?;

        let mut email_field = self.c.find(Locator::Id("identifierId")).await?;
        email_field.send_keys(&email).await?;

        let element = self.c.find(Locator::Id("identifierNext")).await?;
        element.click().await?;

        delay_for(Duration::from_millis(500)).await;

        let mut password_field = self
            .c
            .find(Locator::XPath(r#"//input[@name="password"]"#))
            .await?;
        password_field.send_keys(&secret).await?;

        let element = self.c.find(Locator::Id("passwordNext")).await?;
        element.click().await?;

        Ok(())
    }

    pub async fn like_video(&mut self, url: &str) -> Result<(), error::Error> {
        match self.c.goto(url).await {
            Err(_) => panic!("Webdriver broke"),
            Ok(_) => (),
        };

        let mut element = match self
            .c
            .wait_for_find(Locator::XPath(
                r#"//button[contains(@aria-label, 'like this video')]"#,
            ))
            .await
        {
            Ok(e) => e,
            _ => return Err(Error::LikeButtonNotFound),
        };

        let result = element.attr("aria-pressed").await;

        match result {
            Ok(Some(str)) if str == "false" => match element.click().await {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::LikeButtonNotFound),
            },
            Ok(Some(str)) if str == "true" => Err(Error::AlreadyLiked),
            _ => Err(Error::LikeButtonNotFound),
        }
    }

    pub async fn close(&mut self) -> Result<(), anyhow::Error> {
        self.c.close().await?;

        Ok(())
    }
}
