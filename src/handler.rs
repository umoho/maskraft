use std::io::{Read, Write};

use maskraft::conn::Connection;
use proto::clientbound::{LoginDisconnect, StatusResponse};
use proto::datatype::Encode;
use proto::packet::{Packet, PacketParser};
use proto::serverbound::{Handshake, LoginStart, Serverbound, StatusRequest};

pub(crate) fn handshake<const N: usize>(conn: &mut Connection) {
    let mut buf = [0; N];
    conn.get_stream().read(&mut buf).unwrap();

    let handshake = Packet::decode(&buf, Handshake::decoder).unwrap();
    println!("## [R] Handshake: {:?}", &handshake);

    conn.set_state(handshake.data.next_state.into());
}

pub(crate) fn status<const N: usize>(conn: &mut Connection) {
    let mut buf = [0; N];
    conn.get_stream().read(&mut buf).unwrap();

    let status_request = Packet::decode(&buf, StatusRequest::decoder).unwrap();
    println!("## [R] Status Request: {:?}", &status_request);

    let protocol_version = 765;
    let packet = Packet::new(
        0x00,
        StatusResponse {
            json_response: proto::serde_json::json!({
                "version": {
                    "name": "Any version",
                    "protocol": protocol_version
                },
                "players": {
                    "max": 2024,
                    "online": 0,
                    "sample": []
                },
                "description": {
                    "text": "A pseudo Minecraft server."
                }
            }),
        },
    );
    println!("## [W] Status Response: {:?}", &packet);

    conn.get_stream().write(&packet.encode()).unwrap();
}

pub(crate) fn login<const N: usize>(conn: &mut Connection) {
    let mut buf = [0; N];
    conn.get_stream().read(&mut buf).unwrap();

    let packet_id = PacketParser::new(&buf).packet_id().unwrap();

    if packet_id == 0x00 {
        let login_start = Packet::decode(&buf, LoginStart::decoder).unwrap();
        println!("### [R] Login Start: {:?}", &login_start);

        // refuse login
        let msg = &format!(
            "Sorry {}, this server is not yet prepared to connect.",
            &login_start.data.name
        );
        let disconnect = Packet::new(
            0x00,
            LoginDisconnect {
                reason: proto::serde_json::json!({
                    "text": msg
                }),
            },
        );
        println!("### [W] Disconnect: {:?}", &disconnect);

        conn.get_stream().write(&disconnect.encode()).unwrap();
    }
}