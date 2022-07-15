mod config;
mod password;
mod index;
mod wake;

use crate::{password::password_handler};
use actix_files;
use actix_web::{web, App, HttpServer};
use config::Config;
use std::fs;
use crate::index::index_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let raw_json = fs::read_to_string("./config.json")
        .expect("error reading confgi.json, maybe the file doesn't exist.");
    let config: Config = serde_json::from_str(&raw_json)
        .expect("error reading config.json, it's not well formatted");

    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/wake", web::post().to(password_handler))
            .route("/", web::get().to(index_handler))
            .service(
                actix_files::Files::new("/", "./public")
                    .use_last_modified(false),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
