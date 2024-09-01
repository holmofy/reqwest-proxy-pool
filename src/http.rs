use reqwest::ClientBuilder;

pub(crate) async fn get<T: reqwest::IntoUrl>(url: T) -> reqwest::Result<reqwest::Response> {
    default_client()?.get(url).send().await
}

pub(crate) async fn post<T: reqwest::IntoUrl>(url: T) -> reqwest::Result<reqwest::Response> {
    default_client()?.post(url).send().await
}

pub(crate) fn default_client() -> reqwest::Result<reqwest::Client> {
    Ok(default_client_builder()?.build()?)
}

pub(crate) fn default_client_builder() -> reqwest::Result<ClientBuilder> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0".parse().unwrap());
    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true))
}
