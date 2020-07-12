use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use qrcode::render::svg;
use qrcode::QrCode;

pub async fn qr_code(id: Identity, secret: web::Data<String>) -> impl Responder {
    // Short return
    if !id.identity().is_some() {
        return HttpResponse::Forbidden().finish();
    }

    let code = QrCode::new(secret.get_ref()).unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();

    HttpResponse::Ok().content_type("image/svg+xml").body(image)
}
