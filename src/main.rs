use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_files::Files;
use dotenv::dotenv;
use actix_web::client::Client;
use std::{env};
use std::net::{IpAddr};
use url::Url;

async fn forward_video(
    req: HttpRequest,
    body: web::Bytes,
    url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let mut new_url = url.get_ref().clone();

    new_url.set_query(req.uri().query());

    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let res = forwarded_req.send_body(body).await.unwrap();

    let mut client_resp = HttpResponse::build(res.status());

    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.streaming(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let bind_addr: IpAddr = env::var("BIND_ADDR").unwrap().parse().unwrap();
    let bind_port = env::var("BIND_PORT").unwrap().parse::<u16>().unwrap();
    let stream_base_url = env::var("STREAM_BASE_URL").unwrap().parse::<String>().unwrap();

    let forward_url = Url::parse(&stream_base_url).unwrap();

    HttpServer::new(move || {
        App::new()
        .data(Client::new())
        .data(forward_url.clone())
        .wrap(middleware::Logger::default())
        .service(web::resource("/video").route(web::get().to(forward_video)))
        .service(Files::new("/images", "static/images/").show_files_listing())
        .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind((bind_addr, bind_port))?
    .system_exit()
    .bind("127.0.0.1:8088")?
    .system_exit()
    .run()
    .await
}

