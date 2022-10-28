use tokio::net::UdpSocket;
use trust_dns_server::server::{Request,RequestHandler,ResponseHandler,ResponseInfo};
use trust_dns_server::ServerFuture;
use std::env;


#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let handler = Handler::create_handler();
    let mut server = ServerFuture::new(handler);
    let socket_result = UdpSocket::bind("0.0.0.0:8888").await;
    let socket = match socket_result {
        Ok(sock) => sock,
        Err(err) => panic!("{}", err),
    };
    server.register_socket(socket);
    info!("Starting on port 8888");
    let _ = server.block_until_done().await;
    return ();
}


#[derive(Clone)]
pub struct Handler {}

impl Handler {
    pub fn create_handler() -> Self {
        return Handler{};
    }
}

#[async_trait::async_trait]
impl RequestHandler for Handler {
    async fn handle_request<R: ResponseHandler>(&self, _req: &Request, _res: R) -> ResponseInfo {
        todo!();
    }
}