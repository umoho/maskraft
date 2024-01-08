use maskraft::serv::Server;

fn main() {
    simple_logger::init_with_env().unwrap();
    Server::bind("0.0.0.0:25565").unwrap().serve();
}
