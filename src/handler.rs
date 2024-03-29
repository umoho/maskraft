use std::io::{Read, Write};

use crate::conn::{Connection, State};
use proto::clientbound::{login::Disconnect, status::StatusResponse};
use proto::datatype::Encode;
use proto::packet::{Packet, PacketParser};
use proto::serverbound::{
    handshake::Handshake, login::LoginStart, status::StatusRequest, Serverbound,
};

pub(crate) fn handshake<const N: usize>(conn: &mut Connection) {
    let mut buf = [0; N];
    conn.stream().read(&mut buf).unwrap();

    if buf[0] == 0xfe {
        log::debug!("Received a legacy handshake packet, ignoring it");
        return;
    }

    let handshake = Packet::from_bytes(&buf, Handshake::decoder).unwrap();
    log::trace!("(recv) Handshake: {:?}", &handshake);

    let next_state = Into::<i32>::into(handshake.data.next_state);
    conn.switch_state(State::try_from(next_state).unwrap());
}

pub(crate) fn status<const N: usize>(conn: &mut Connection) {
    let mut buf = [0; N];
    conn.stream().read(&mut buf).unwrap();

    let status_request = Packet::from_bytes(&buf, StatusRequest::decoder).unwrap();
    log::trace!("(recv) Status Request: {:?}", &status_request);

    let protocol_version = 765;
    let packet = Packet::new(StatusResponse {
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
    });
    conn.stream().write(&packet.encode()).unwrap();
    log::trace!("(sent) Status Response: {:?}", &packet);
}

pub(crate) fn login<const N: usize>(conn: &mut Connection) {
    let mut buf = [0; N];
    conn.stream().read(&mut buf).unwrap();

    let packet_id = PacketParser::new(&buf).packet_id().unwrap();

    if packet_id == 0x00 {
        let login_start = Packet::from_bytes(&buf, LoginStart::decoder).unwrap();
        log::trace!("(recv) Login Start: {:?}", &login_start);

        std::thread::sleep(std::time::Duration::from_secs(3));
        // refuse login
        let msg = &format!(
            "Sorry {}, this server is not yet prepared to connect.",
            &login_start.data.name
        );
        let disconnect = Packet::new(Disconnect {
            reason: proto::serde_json::json!({
                "text": msg
            }),
        });
        conn.stream().write(&disconnect.encode()).unwrap();
        log::trace!("(sent) Disconnect: {:?}", &disconnect);
    }
}
