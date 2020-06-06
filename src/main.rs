use actix_files::Files;
use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpServer};
use bombolone::configs::config::get_app_config;
use bombolone::handlers::video::forward_video;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_app_config().clone();

    // Need to borrow as config is moved into Factory Closure
    let bind_addr = config.bind_addr;
    let bind_port = config.bind_port;

    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .data(config.forward_url.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/video").route(web::get().to(forward_video)))
            .service(Files::new("/images", &config.image_root).show_files_listing())
            .service(Files::new("/", &config.web_root).index_file("index.html"))
    })
    .bind((bind_addr, bind_port))?
    .system_exit()
    .bind("127.0.0.1:8088")?
    .system_exit()
    .run()
    .await
}
