use pingora::prelude::{Opt, Server};

fn main() {
    let mut gateway_server = Server::new(None).unwrap();
    gateway_server.bootstrap();
    gateway_server.run_forever();
}