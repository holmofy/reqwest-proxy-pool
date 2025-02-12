use crate::proxy::Proxy;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProxyError>;

/// Scraping Error
#[derive(Error, Debug)]
pub enum ProxyError {
    #[error(transparent)]
    ReqwestErr(#[from] reqwest::Error),

    #[error(transparent)]
    ScraperErr(#[from] reqwest_scraper::error::ScraperError),

    #[error("protocol parse error: {0}")]
    ProtocolParseErr(String),

    #[error(transparent)]
    SendErr(#[from] tokio::sync::mpsc::error::SendError<Proxy>),

    #[error(transparent)]
    JsonDecodeErr(#[from] serde_json::error::Error),

    #[error(transparent)]
    RedisErr(#[from] redis::RedisError),

    #[error("check proxy {0} failed: {1}")]
    ProxyCheckErr(Proxy, reqwest::Error),
}
