/// * https://www.kuaidaili.com/free/intr/
use super::{OkLogErr, ProxySender, SendProxyEx};
use crate::http;
use crate::{
    error::Result,
    proxy::{IntoProxy, Privacy, ProxyType},
    utils::substr_between,
};
use async_trait::async_trait;
use serde::Deserialize;
use std::{net::SocketAddr, str::FromStr};

#[derive(Debug, Deserialize)]
pub(super) struct FreeProxy {
    ip: String,
    port: String,
}

impl IntoProxy for FreeProxy {
    fn make_proxy(self) -> Option<crate::proxy::Proxy> {
        let socket = SocketAddr::from_str(&format!("{}:{}", self.ip, self.port)).ok_log_err()?;

        Some(crate::proxy::Proxy {
            socket,
            ty: ProxyType::Http,
            pri: Privacy::HighAnonymity,
        })
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct HttpsProxy {
    ip: String,
    port: String,
}

impl IntoProxy for HttpsProxy {
    fn make_proxy(self) -> Option<crate::proxy::Proxy> {
        let socket = SocketAddr::from_str(&format!("{}:{}", self.ip, self.port)).ok_log_err()?;

        Some(crate::proxy::Proxy {
            socket,
            ty: ProxyType::Https,
            pri: Privacy::HighAnonymity,
        })
    }
}

pub(super) struct ProxyFetcher;

#[async_trait]
impl super::ProxyFetcher for ProxyFetcher {
    async fn fetch(&self, sender: ProxySender) -> Result<()> {
        let _ = inner_fetch("https://www.kuaidaili.com/free/inha/", &sender).await?; //高匿名
        let _ = inner_fetch("https://www.kuaidaili.com/free/intr/", &sender).await?; //普匿名
        inner_fetch("https://www.kuaidaili.com/free/fps/", &sender).await //海外高匿
    }
}

async fn inner_fetch(url: &str, sender: &ProxySender) -> Result<()> {
    let html = http::get(url).await?.text().await?;

    let json = match substr_between(&html, "const fpsList = ", ";") {
        Some(json) => json,
        None => {
            return Ok(log::warn!("json not found"));
        }
    };

    let items: Vec<FreeProxy> = serde_json::from_str(json)?;

    for item in items {
        log::trace!("fetch proxy: {:?}", item);
        sender.send_proxy(item).await?;
    }
    Ok(())
}

inventory::submit! {
    &ProxyFetcher as &dyn super::ProxyFetcher
}
