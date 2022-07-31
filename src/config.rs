use serde::Deserialize;
use std::{collections::HashMap};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub password: String,
    pub mac: [u8; 6],
    pub port: u16,
}

pub fn generate_config() -> Config {
    let password = std::env::var("PASSWORD").expect("set password");
    let mac = std::env::var("MAC_ADDRESS").expect("set mac address");
    let mac = hex(&mac);
    let port: u16 = std::env::var("PORT")
        .expect("set port")
        .parse()
        .expect("invalid port");

    Config {
        password,
        mac,
        port,
    }
}

pub fn hex(input: &str) -> [u8; 6] {
    let mut mapping: HashMap<char, u8> = HashMap::new();

    mapping.insert('0', 0);
    mapping.insert('1', 1);
    mapping.insert('2', 2);
    mapping.insert('3', 3);
    mapping.insert('4', 4);
    mapping.insert('5', 5);
    mapping.insert('6', 6);
    mapping.insert('7', 7);
    mapping.insert('8', 8);
    mapping.insert('9', 9);
    mapping.insert('a', 10);
    mapping.insert('b', 11);
    mapping.insert('c', 12);
    mapping.insert('d', 13);
    mapping.insert('e', 14);
    mapping.insert('f', 15);

    let mut h = input.chars();
    let mut mac_address: [u8; 6] = [0, 0, 0, 0, 0, 0];

    for iter in 0..6 {
        let dig1 = mapping[&h.next().expect("error parsing hex")];
        let dig2 = mapping[&h.next().expect("error parsing hex")];
        mac_address[iter] = dig1 * 16 + dig2;
    }

    mac_address
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn to_mac() {
        let expect_result: [u8; 6] = [178, 129, 22, 243, 30, 108];
        assert_eq!(hex("b28116f31e6c"), expect_result);
    }
}
