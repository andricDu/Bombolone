use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::{env};
use std::net::{IpAddr};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let bind_addr: IpAddr = env::var("BIND_ADDR").unwrap().parse().unwrap();
    let bind_port = env::var("BIND_PORT").unwrap().parse::<u16>().unwrap();

    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind((bind_addr, bind_port))?
    .system_exit()
    .bind("127.0.0.1:8088")?
    .system_exit()
    .run()
    .await
}
