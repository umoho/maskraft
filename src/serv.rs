use std::net::{TcpListener, ToSocketAddrs};

use crate::conn::{Connection, State};
use crate::handler;

pub struct Server {
    socket: TcpListener,
}

impl Server {
    pub fn bind<A>(addr: A) -> std::io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        let socket = TcpListener::bind(addr)?;
        Ok(Self { socket })
    }

    pub fn serve(&self) {
        log::info!("Listing on {}", self.socket.local_addr().unwrap());
        for stream in self.socket.incoming() {
            std::thread::spawn(move || {
                let stream = stream.unwrap();
                log::debug!("Accepted client from {}", stream.peer_addr().unwrap());

                Connection::new(stream)
                    .handle(handler::handshake::<0xff>)
                    .handle_when(State::Status, handler::status::<0xff>)
                    .handle_when(State::Login, handler::login::<0xff>);
            })
            .join()
            .unwrap();
        }
    }
}
