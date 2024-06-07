use std::{error::Error, net::Ipv4Addr, sync::Arc};

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use soundclone_db::repositories::database::Database;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!("Starting HTTP server at http://localhost:8080");

    let db = Arc::new(Database::default());
    let user_db = Arc::new(soundclone_db::repositories::user_repository::Users::new(
        Arc::clone(&db),
    ));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Arc::clone(&user_db)))
            .service(soundclone_service::handlers::user_handler::register)
            .service(soundclone_service::handlers::user_handler::login)
            .route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
