use actix_files::Files;
use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpServer};
use bombolone::config::config::get_app_config;
use bombolone::handler::qrcode::qr_code;
use bombolone::handler::video::forward_video;
use qrcode::QrCode;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_app_config().clone();

    // Need to borrow as config is moved into Factory Closure
    let bind_addr = config.bind_addr;
    let bind_port = config.bind_port;

    // Dump QR Code to stdout
    let code = QrCode::new(&config.app_secret).unwrap();
    let qr_string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .build();
    println!("{}", qr_string);

    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .data(config.forward_url.clone())
            .data(config.app_secret.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/qrcode.svg").route(web::get().to(qr_code)))
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
