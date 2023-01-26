use actix_web::{web, HttpResponse, Responder};
use argon2::verify_encoded;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{config::Config, wake::*};

fn default_hash<T>(inp: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    inp.hash(&mut hasher);
    hasher.finish()
}

pub async fn password_handler(password: String, config: web::Data<Config>) -> impl Responder {
    let res;

    if config.hashed {
        res = verify_encoded(&config.password, password.as_bytes()).unwrap();
    } else {
        res = default_hash(&password) == default_hash(&config.password);
    }

    if res {
        MagicPacket::new(&config.mac).send();
        return HttpResponse::Ok().body("true");
    }
    HttpResponse::Ok().body("false")
}
