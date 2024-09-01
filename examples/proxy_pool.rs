use reqwest_proxy_pool::ProxyPool;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "INFO,reqwest_proxy_pool=TRACE");
    env_logger::init();

    let redis = redis::Client::open("redis://localhost")
        .expect("redis open failed")
        .get_connection_manager()
        .await
        .expect("build connect manager failed");

    let mut pp = ProxyPool::new(redis);

    pp.scraper().await.expect("scraper proxy ip failed");
}
