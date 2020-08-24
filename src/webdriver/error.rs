use std::error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    WebdriverNotFound,
    SignInFailed,
    AlreadyLiked,
    LikeButtonNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::WebdriverNotFound => write!(f, "Webdriver not found"),
            Error::SignInFailed => write!(f, "Sign-in failed, check email and secret"),
            Error::AlreadyLiked => write!(f, "Video already liked"),
            Error::LikeButtonNotFound => {
                write!(f, "Like button not found, has the video been taken down?")
            }
        }
    }
}

impl error::Error for Error {}
