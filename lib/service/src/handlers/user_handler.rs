use {
    actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse},
    log::{debug, error},
    serde::{Deserialize, Serialize},
    soundclone_db::repositories::*,
    std::sync::Arc,
};

#[derive(Debug, Serialize, Deserialize)]
struct AuthDTO {
    email: String,
    password: String,
}

#[actix_web::post("/register")]
async fn register(
    req: HttpRequest,
    db: web::Data<Arc<user_repository::Users>>,
    body: web::Json<AuthDTO>,
) -> HttpResponse {
    debug!("REQ: {req:?}");

    HttpResponse::Ok()
        .content_type("application/json")
        .status(StatusCode::OK)
        .json(body)
}

#[actix_web::post("/login")]
async fn login(
    req: HttpRequest,
    db: web::Data<Arc<user_repository::Users>>,
    body: web::Json<AuthDTO>,
) -> HttpResponse {
    debug!("REQ: {req:?}");

    HttpResponse::Ok()
        .content_type("application/json")
        .status(StatusCode::OK)
        .json(body)
}
