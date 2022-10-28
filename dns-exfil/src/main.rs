use tokio::net::UdpSocket;
use trust_dns_server::client::rr::LowerName;
use trust_dns_server::server::{Request,RequestHandler,ResponseHandler,ResponseInfo,ServerFuture};
use trust_dns_server::proto::op::{OpCode, MessageType, Header, ResponseCode};
use trust_dns_server::proto::rr::{RData, Record};
use trust_dns_server::authority::{MessageResponseBuilder};
use std::str::FromStr;
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
    let handler = Handler::create_handler("disna-m.top", "exfil.disna-m.top");
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
pub struct Handler {
    pub root_zone: LowerName,
    pub exfil_zone: LowerName,
}

impl Handler {
    pub fn convert_to_lowername(s: &str) -> LowerName {
        let result = LowerName::from_str(s);
        match result {
            Ok(v) => return v,
            Err(_) => panic!("lowername conversion failed")
        }
    }
    
    pub fn create_handler(root_zone: &str, exfil_zone: &str) -> Self {
        return Handler{
            root_zone: Handler::convert_to_lowername(root_zone),
            exfil_zone: Handler::convert_to_lowername(exfil_zone),
        };
    }
    
    async fn parse_request<R: ResponseHandler>(&self, req: &Request, res: R) -> Result<ResponseInfo, Error> {
        if req.op_code() != OpCode::Query {
            return Err(Error::InvalidOpCode(req.op_code()));
        }
        if req.message_type() != MessageType::Query {
            return Err(Error::InvalidMessageType(req.message_type()));
        }
        match req.query().name() {
            name if self.exfil_zone.zone_of(name) => {
                return self.parse_request_exfil(req, res).await;
            }
            name if self.root_zone.zone_of(name) => {
                return self.parse_request_root(req, res).await;
            }
            name => Err(Error::InvalidZone(name.clone()))
        }
    }

    async fn parse_request_root<R: ResponseHandler>(&self, req: &Request, mut res: R) -> Result<ResponseInfo, Error> {
        let query_name = req.query().name().to_string();
        info!("Root query: {}", query_name);
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

    async fn parse_request_exfil<R: ResponseHandler>(&self, req: &Request, mut res: R) -> Result<ResponseInfo, Error> {
        let query_name = req.query().name().to_string();
        info!("Exfil query: {}", query_name);
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
    #[error("Invalid Zone {0:}")]
    InvalidZone(LowerName),
    #[error("IO error: {0:}")]
    Io(#[from] std::io::Error),
    #[error("crap")]
    Error()

}