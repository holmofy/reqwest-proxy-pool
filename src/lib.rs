mod error;
mod http;
mod proxy;
mod utils;

use crate::error::Result;
use chrono::{Local, NaiveDateTime};
use proxy::Proxy;
use redis::AsyncCommands;
use reqwest::{StatusCode, Url};
use std::time::Duration;
use tokio::sync::mpsc;

static POOL_REDIS_SET_KEY: &str = "ip_proxy_pool:zset";
static POOL_REDIS_STREAM_KEY: &str = "ip_proxy_pool:stream";

pub struct ProxyPool {
    redis: redis::aio::ConnectionManager,
}

impl ProxyPool {
    pub fn new(redis: redis::aio::ConnectionManager) -> Self {
        Self { redis }
    }

    pub fn choose(&self) -> Option<Url> {
        reqwest::Url::parse("https://my.prox").ok()
    }

    pub async fn scraper(&mut self) -> Result<()> {
        let redis = &mut self.redis;
        let (tx, mut rx) = mpsc::channel::<Proxy>(32);
        proxy::fetch(tx).await;

        while let Some(proxy) = rx.recv().await {
            let proxy = proxy.to_string();
            let now = Local::now().naive_utc().to_string();
            let resp: String = redis.hset(&proxy, "updated", now).await?;
            log::info!("set proxy ip {} to redis:{}", proxy, resp);
        }

        Ok(())
    }

    pub async fn check(&self) {}

    async fn check_proxy(proxy: Proxy) -> Result<ProxyStatus> {
        let proxy_url = proxy.build_url();
        let proxy_target = reqwest::Proxy::custom(move |_| Some(proxy_url.clone()));
        let client = http::default_client_builder()?
            .proxy(proxy_target)
            .build()?;
        let resp = client
            .head("https://www.ip.cn/api/index?ip=&type=0")
            .send()
            .await;
        match resp {
            Ok(resp) => {
                if resp.status() == StatusCode::OK {
                    let now = Local::now().naive_local();
                    Ok(ProxyStatus {
                        checked: now,
                        resp_time: Duration::from_millis(1000),
                    })
                } else {
                    println!("{:?}", resp);
                    todo!()
                }
            }
            Err(e) => Err(error::ProxyError::ProxyCheckErr(proxy, e)),
        }
    }
}

#[derive(Debug)]
struct ProxyStatus {
    checked: NaiveDateTime,
    resp_time: Duration,
}

mod tests {

    #[tokio::test]
    async fn test_check_proxy() {
        use crate::{
            proxy::{Privacy, Proxy, ProxyType},
            ProxyPool,
        };

        let proxy = Proxy::new(
            ([44, 226, 167, 102], 3128),
            ProxyType::Http,
            Privacy::HighAnonymity,
        );
        let r = ProxyPool::check_proxy(proxy).await;
        println!("result {:?}", r);
    }
}
