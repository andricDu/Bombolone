use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub secret: String,
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    secret: web::Data<String>,
) -> Result<HttpResponse, Error> {
    let user_secret = &auth_data.secret;
    let secret = secret.get_ref();

    if secret == user_secret {
        id.remember(user_secret.to_string());
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}
