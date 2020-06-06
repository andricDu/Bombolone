use dotenv::dotenv;
use std::env;
use std::net::IpAddr;
use url::Url;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: IpAddr,
    pub bind_port: u16,
    pub forward_url: Url,
    pub image_root: String,
    pub web_root: String,
}

pub fn get_app_config() -> AppConfig {
    dotenv().ok();

    let bind_addr: IpAddr = env::var("BIND_ADDR").unwrap().parse().unwrap();
    let bind_port: u16 = env::var("BIND_PORT").unwrap().parse().unwrap();
    let stream_base_url: String = env::var("STREAM_BASE_URL").unwrap().parse().unwrap();
    let forward_url = Url::parse(&stream_base_url).unwrap();

    let image_root: String = env::var("STATIC_IMAGE_ROOT").unwrap().parse().unwrap();
    let web_root: String = env::var("STATIC_WEB_ROOT").unwrap().parse().unwrap();

    AppConfig {
        bind_addr,
        bind_port,
        forward_url,
        image_root,
        web_root,
    }
}
