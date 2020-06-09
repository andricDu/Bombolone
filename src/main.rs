use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::client::Client;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use bombolone::config::config::get_app_config;
use bombolone::handler::auth::login;
use bombolone::handler::qrcode::qr_code;
use bombolone::handler::video::forward_video;
use qrcode::QrCode;

async fn test_auth(id: Identity) -> Result<HttpResponse, Error> {
    if id.identity().is_some() {
        Ok(HttpResponse::Ok().body("Cool"))
    } else {
        Ok(HttpResponse::Forbidden().body("Fuck off"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_app_config().clone();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();

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
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(config.signing_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&config.domain)
                    .max_age_time(chrono::Duration::days(365))
                    .secure(false), // this can only be true if you have https
            ))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/test_auth").route(web::get().to(test_auth)))
            .service(web::resource("/qrcode.svg").route(web::get().to(qr_code)))
            .service(web::resource("/video").route(web::get().to(forward_video)))
            .service(Files::new("/", &config.web_root).index_file("index.html"))
    })
    .bind((bind_addr, bind_port))?
    .system_exit()
    .bind("127.0.0.1:8088")?
    .system_exit()
    .run()
    .await
}
