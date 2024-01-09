use crate::{
    datatype::{UnsignedShort, VarInt},
    packet::BytesParser,
    Result,
};

use super::Serverbound;

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: UnsignedShort,
    pub next_state: VarInt,
}

impl Serverbound for Handshake {
    fn decoder(parser: &mut BytesParser) -> Result<Self> {
        let protocol_version = parser.next()?;
        let server_address = parser.next()?;
        let server_port = parser.next()?;
        let next_state = parser.next()?;

        Ok(Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

#[test]
fn test_handshake() {
    use crate::packet::Packet;

    let bytes = vec![
        0x10, 0x0, 0xf6, 0x5, 0x9, 0x31, 0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31, 0x63,
        0xdd, 0x1, 0x1, 0x0,
    ];
    let handshake_packet = Packet::from_bytes(&bytes, Handshake::decoder);
    println!("{:?}", handshake_packet);
}
