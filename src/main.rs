extern crate dotenv;

use dotenv::dotenv;

mod config;
mod index;
mod password;
mod wake;

use crate::index::index_handler;
use crate::password::password_handler;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::generate_config();

    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/wake", web::post().to(password_handler))
            .route("/", web::get().to(index_handler))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
