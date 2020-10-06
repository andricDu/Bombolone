use actix_cors::Cors;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::client::Client;
use actix_web::cookie::SameSite;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use bombolone::config::config::get_app_config;
use bombolone::handler::auth::login;
use bombolone::handler::qrcode::qr_code;
use bombolone::handler::video::forward_video;
use qrcode::QrCode;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

async fn test_auth(id: Identity) -> Result<HttpResponse, Error> {
    if id.identity().is_some() {
        Ok(HttpResponse::Ok().body("Cool"))
    } else {
        Ok(HttpResponse::Forbidden().body("Go Away"))
    }
}

fn build_tls_config(cert_file: &String, key_file: &String) -> ServerConfig {
    let mut tls_config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(cert_file).unwrap());
    let key_file = &mut BufReader::new(File::open(key_file).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    tls_config
        .set_single_cert(cert_chain, keys.remove(0))
        .unwrap();

    tls_config
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

    // load ssl keys
    let tls_config = build_tls_config(&config.cert_file, &config.key_file);

    // Dump QR Code to stdout
    let code = QrCode::new(&config.app_secret).unwrap();
    let qr_string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(3, 1)
        .build();
    println!("{}", qr_string);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .data(Client::new())
            .data(config.forward_url.clone())
            .data(config.app_secret.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(config.signing_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&config.domain)
                    .max_age_time(time::Duration::days(365))
                    .same_site(SameSite::Strict)
                    .secure(false), // this can only be true if you have https
            ))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/test_auth").route(web::get().to(test_auth)))
            .service(web::resource("/qrcode.svg").route(web::get().to(qr_code)))
            .service(web::resource("/video").route(web::get().to(forward_video)))
            .service(Files::new("/", &config.web_root).index_file("index.html"))
    })
    .bind_rustls((bind_addr, bind_port), tls_config)?
    .system_exit()
    .run()
    .await
}
