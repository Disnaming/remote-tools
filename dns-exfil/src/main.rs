pub mod delegator;
pub mod error;

use crate::error::Error;
use crate::delegator::Delegator;

use tokio::net::UdpSocket;
use trust_dns_server::server::{Request,RequestHandler,ResponseHandler,ResponseInfo,ServerFuture};
use trust_dns_server::proto::op::{OpCode, MessageType, Header, ResponseCode};
use std::env;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = "8888";
    let addr = format!("{}:{}", host, port);
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let handler = MainHandler::create_request_handler();
    let mut server = ServerFuture::new(handler);
    let socket_result = UdpSocket::bind(addr).await;
    let socket = match socket_result {
        Ok(sock) => sock,
        Err(err) => panic!("{}", err),
    };
    server.register_socket(socket);
    info!("Starting on port {}", port);
    let _ = server.block_until_done().await;
    return ();
}

#[derive(Clone, Debug)]
pub struct MainHandler {
    delegator: Delegator,
}

impl MainHandler {
    pub fn create_request_handler() -> Self {
        return MainHandler{
            delegator: Delegator::new("disna-m.top"),
        };
    }
    
    async fn parse_request<R: ResponseHandler>(&self, req: &Request, res: R) -> Result<ResponseInfo, Error> {
        if req.op_code() != OpCode::Query {
            return Err(Error::InvalidOpCode(req.op_code()));
        }
        if req.message_type() != MessageType::Query {
            return Err(Error::InvalidMessageType(req.message_type()));
        }
        return self.delegator.handle_request(req, res).await;
    }
}

#[async_trait::async_trait]
impl RequestHandler for MainHandler {
    async fn handle_request<R: ResponseHandler>(&self, req: &Request, res: R) -> ResponseInfo {
        match self.parse_request(req, res).await {
            Ok(v) => return v,
            Err(err) => {
                error!("Error in Request Handler: {err}");
                let mut header = Header::new();
                header.set_response_code(ResponseCode::ServFail);
                header.into()
            }
        }
    }
}