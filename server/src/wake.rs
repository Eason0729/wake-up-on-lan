use std::net::{Ipv4Addr, UdpSocket};

const SIX_FF: [u8; 6] = [0xFF; 6];

// [u8; 102]
pub fn to_magic_bytes(mac_address: &[u8; 6]) -> Vec<u8> {
    let dst: &mut Vec<u8> = &mut vec![0_u8; 0];

    dst.extend_from_slice(&SIX_FF);

    (0..16).for_each(|_iter| {
        dst.extend_from_slice(mac_address);
    });

    dst.to_owned()
}

pub struct MagicPacket {
    packet: Vec<u8>,
}

impl MagicPacket {
    pub fn new(mac_address: &[u8; 6]) -> MagicPacket {
        MagicPacket {
            packet: to_magic_bytes(&mac_address),
        }
    }
    pub fn send(&self) {
        println!("user  : sending MagicPacket");
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        socket.set_broadcast(true).unwrap();
        socket
            .send_to(&self.packet, (Ipv4Addr::BROADCAST, 9))
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn to_magic_packet_test() {
        assert_eq!(
            to_magic_bytes(&[1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8]),
            [
                255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8,
                1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8,
                3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8,
                5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8,
                1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8,
                3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8,
                5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8,
                1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8
            ]
            .to_vec()
        );
    }
}
