use std::net::TcpListener;
use std::thread;

use maskraft::conn::Connection;

mod handler;

fn main() {
    let server_socket = TcpListener::bind("0.0.0.0:25565").unwrap();
    println!(
        "# Server listening on {}",
        server_socket.local_addr().unwrap()
    );
    for stream in server_socket.incoming() {
        thread::spawn(move || {
            let stream = stream.unwrap();
            println!("# Accepted client: {:?}", stream.peer_addr().unwrap());

            Connection::new(stream)
                .handle(handler::handshake::<0xff>)
                .handle_when(0x01, handler::status::<0xff>)
                .handle_when(0x02, handler::login::<0xff>);
        })
        .join()
        .unwrap();
    }
}
