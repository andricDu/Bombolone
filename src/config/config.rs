use dotenv::dotenv;
use std::env;
use std::net::IpAddr;
use url::Url;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: IpAddr,
    pub bind_port: u16,
    pub domain: String,
    pub forward_url: Url,
    pub web_root: String,
    pub app_secret: String,
    pub signing_key: String,
}

pub fn get_app_config() -> AppConfig {
    dotenv().ok();

    let bind_addr: IpAddr = env::var("BIND_ADDR").unwrap().parse().unwrap();
    let bind_port: u16 = env::var("BIND_PORT").unwrap().parse().unwrap();
    let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_string());

    let stream_base_url: String = env::var("STREAM_BASE_URL").unwrap().parse().unwrap();
    let forward_url = Url::parse(&stream_base_url).unwrap();

    let web_root: String = env::var("STATIC_WEB_ROOT").unwrap().parse().unwrap();

    let app_secret: String = env::var("APP_SECRET").unwrap().parse().unwrap();

    let signing_key: String = env::var("SIGNING_KEY").unwrap().parse().unwrap();

    AppConfig {
        bind_addr,
        bind_port,
        domain,
        forward_url,
        web_root,
        app_secret,
        signing_key,
    }
}
