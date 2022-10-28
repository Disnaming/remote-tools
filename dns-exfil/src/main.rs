use tokio::net::UdpSocket;
use trust_dns_server::server::{Request,RequestHandler,ResponseHandler,ResponseInfo,ServerFuture};
use trust_dns_server::proto::op::{OpCode, MessageType, Header, ResponseCode};
use trust_dns_server::proto::rr::{RData, Record};
use trust_dns_server::authority::{MessageResponseBuilder};
use std::{env, net::IpAddr};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = "8888";
    let addr = format!("{}:{}", host, port);
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let handler = Handler::create_handler();
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
pub struct Handler {}

impl Handler {
    pub fn create_handler() -> Self {
        return Handler{};
    }
    
    #[allow(unused_mut)]
    async fn parse_request<R: ResponseHandler>(&self, req: &Request, mut res: R) -> Result<ResponseInfo, Error> {
        if req.op_code() != OpCode::Query {
            return Err(Error::InvalidOpCode(req.op_code()));
        }
        if req.message_type() != MessageType::Query {
            return Err(Error::InvalidMessageType(req.message_type()));
        }
        let query_name = req.query().name().to_string();
        debug!("Query name: {}", query_name);
        let builder = MessageResponseBuilder::from_message_request(req);
        let mut header = Header::response_from_request(req.header());
        let rdata = match req.src().ip() {
            IpAddr::V4(ipv4) => RData::A(ipv4),
            IpAddr::V6(ipv6) => RData::AAAA(ipv6),
        };       
        let records = vec![Record::from_rdata(req.query().name().into(), 60, rdata)];
        header.set_authoritative(true);
        let response = builder.build(header, records.iter(), &[], &[], &[]);
        let result = res.send_response(response).await;
        match result {
            Ok(v) => return Ok(v),
            Err(_) => return Err(Error::Error()),
        }
    }
}

#[async_trait::async_trait]
impl RequestHandler for Handler {
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Opcode {0:}")]
    InvalidOpCode(OpCode),
    #[error("Invalid MessageType {0:}")]
    InvalidMessageType(MessageType),
    // #[error("Invalid Zone {0:}")]
    // InvalidZone(LowerName),
    #[error("IO error: {0:}")]
    Io(#[from] std::io::Error),
    #[error("crap")]
    Error()

}