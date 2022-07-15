use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub password: String,
    pub mac: [u8; 6],
    pub port: u16,
}
