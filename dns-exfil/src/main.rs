pub mod delegator;
pub mod dns_request_handler;
pub mod error;

use dns_exfil::start_server;

#[tokio::main]
async fn main() {
    // create logging runtime
    env_logger::init();
    let host = "0.0.0.0";
    let port = "8888";
    let domain = "disna-m.top";
    let server = start_server(host, port, domain).await;
    let _ = server.block_until_done().await;
}
