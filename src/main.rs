use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use proto::clientbound::{LoginDisconnect, StatusResponse};
use proto::datatype::Encode;
use proto::packet::{Packet, PacketParser};
use proto::serverbound::{Handshake, LoginStart, Serverbound, StatusRequest};

pub enum Response {
    Skip,
    SetState(i32),
    WriteBuf(Vec<u8>),
}

pub struct ClientConnection {
    stream: TcpStream,
    state: i32,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream, state: 0 }
    }

    fn recv(&mut self) -> Vec<u8> {
        let mut buf = [0; 0x7f];
        match self.stream.read(&mut buf) {
            Ok(_) => (),
            Err(e) => eprintln!("Error while reading from stream. {}", e),
        }
        buf.to_vec()
    }

    pub fn begin_handshake(mut self) -> Self {
        let handshake = Packet::decode(&self.recv(), Handshake::decoder).unwrap();
        println!("## [R] Handshake: {:?}", &handshake);
        self.state = handshake.data.next_state.0;
        self
    }

    pub fn when<F>(mut self, state: i32, handler: F) -> Self
    where
        F: Fn(&[u8]) -> Response,
    {
        if state == self.state {
            let resp = handler(&self.recv());
            match resp {
                Response::Skip => (),
                Response::SetState(state) => self.state = state,
                Response::WriteBuf(buf) => {
                    let _ = self.stream.write(&buf[..]).unwrap();
                }
            }
        }
        self
    }
}

fn handle_status(buf: &[u8]) -> Response {
    let status_request = Packet::decode(&buf, StatusRequest::decoder).unwrap();
    println!("## [R] Status Request: {:?}", &status_request);

    let protocol_version = 764;
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

    Response::WriteBuf(packet.encode())
}

fn handle_login(buf: &[u8]) -> Response {
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
        return Response::WriteBuf(disconnect.encode());
    }

    Response::Skip
}

fn main() {
    let server_socket = TcpListener::bind("0.0.0.0:25565").unwrap();
    println!(
        "# Server listening on {}",
        server_socket.local_addr().unwrap()
    );
    for stream in server_socket.incoming() {
        thread::spawn(|| {
            let stream = stream.unwrap();
            println!("# Accepted client: {:?}", stream.peer_addr().unwrap());

            ClientConnection::new(stream)
                .begin_handshake()
                .when(0x01, handle_status)
                .when(0x02, handle_login);
        })
        .join()
        .unwrap();
    }
}
