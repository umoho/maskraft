use crate::{datatype::VarInt, packet::BytesParser, Result};

pub trait Serverbound: Sized {
    fn decoder(parser: &mut BytesParser) -> Result<Self>;
}

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
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
    let handshake_packet = Packet::decode(&bytes, Handshake::decoder);
    println!("{:?}", handshake_packet);
}

#[derive(Debug)]
pub struct StatusRequest;

impl Serverbound for StatusRequest {
    fn decoder(_: &mut BytesParser) -> Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct StatusPingRequest {
    pub payload: i64,
}

impl Serverbound for StatusPingRequest {
    fn decoder(parser: &mut BytesParser) -> Result<Self> {
        let payload = parser.next()?;
        Ok(Self { payload })
    }
}

#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
    // pub player_uuid: (), // todo: uuid decoding
}

impl Serverbound for LoginStart {
    fn decoder(parser: &mut BytesParser) -> Result<Self> {
        let name = parser.next()?;
        Ok(Self { name })
    }
}
