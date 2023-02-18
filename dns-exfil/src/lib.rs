pub mod delegator;
pub mod dns_request_handler;
pub mod error;

use tokio::net::UdpSocket;
use trust_dns_server::server::ServerFuture;

pub async fn start_server(
    host: &str,
    port: &str,
    domain: &str,
) -> ServerFuture<dns_request_handler::DNSRequestHandler> {
    env_logger::init();
    let addr = format!("{}:{}", host, port);
    let handler = dns_request_handler::DNSRequestHandler::create_request_handler(domain);
    let mut server = ServerFuture::new(handler);
    let socket_result = UdpSocket::bind(addr).await;
    let socket = match socket_result {
        Ok(sock) => sock,
        Err(err) => panic!("{}", err),
    };
    server.register_socket(socket);
    return server;
}

pub fn stop_server(server: ServerFuture<dns_request_handler::DNSRequestHandler>) {
    drop(server);
    return ();
}
