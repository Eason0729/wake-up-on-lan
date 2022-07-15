use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{config::Config, wake::*};

#[derive(Deserialize)]
pub struct RequestData {
    password: String,
}

#[derive(Serialize)]
pub struct RespondData {
    status: bool,
}

pub async fn password_handler(
    data: web::Json<RequestData>,
    config: web::Data<Config>,
) -> impl Responder {
    let mut hasher = DefaultHasher::new();
    (data.password).hash(&mut hasher);
    let left = hasher.finish();

    let mut hasher = DefaultHasher::new();
    (config.password).hash(&mut hasher);
    let right = hasher.finish();

    if left == right {
        MagicPacket::new(&config.mac).send();
        return web::Json(RespondData { status: true });
    }

    web::Json(RespondData { status: false })
}
