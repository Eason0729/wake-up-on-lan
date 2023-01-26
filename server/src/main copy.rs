extern crate dotenv;

use dotenv::dotenv;

mod config;
mod index;
mod password;
mod wake;

use crate::{index::index_handler, password::password_handler};
use actix_web::{web, App, HttpServer};
use std::{net::Ipv4Addr, str::FromStr};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::generate_config();

    let port = config.port;

    let addr = match std::env::var("ADDRESS") {
        Ok(x) => Ipv4Addr::from_str(&x).expect("error parsing ADDRESS"),
        Err(_) => Ipv4Addr::UNSPECIFIED,
    };

    println!("server: start serving at {:?}:{:?}", addr, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/wake", web::post().to(password_handler))
            .route("/", web::get().to(index_handler))
    })
    .bind((addr, port))?
    .run()
    .await
}
