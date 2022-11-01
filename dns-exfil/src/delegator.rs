pub mod exfil;
pub mod root;
pub mod message;

use crate::error::Error;
use crate::delegator::exfil::ExfilHandler;
use crate::delegator::root::RootHandler;
use crate::delegator::message::MessageHandler;


use trust_dns_server::client::rr::LowerName;
use trust_dns_server::server::{Request,ResponseHandler,ResponseInfo};
use trust_dns_server::proto::op::{Header};
use trust_dns_server::proto::rr::{RData, Record};
use trust_dns_server::authority::{MessageResponseBuilder};
use std::str::FromStr;
use std::net::IpAddr;

#[async_trait::async_trait]
pub trait BaseHandler {
    async fn build_basic_response<R: ResponseHandler>(req: &Request, res: R) -> Result<ResponseInfo, Error>;
}

#[async_trait::async_trait]
pub trait Handler: BaseHandler {
    async fn handle_request<R: ResponseHandler>(&self, req: &Request, mut res: R) -> Result<ResponseInfo, Error>;
    fn get_zone(&self) -> &LowerName;
}

#[async_trait::async_trait]
impl <T: Handler> BaseHandler for T {
    async fn build_basic_response<R: ResponseHandler>(req: &Request, mut res: R) -> Result<ResponseInfo, Error> {
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


// #[derive(Clone, Debug)]
pub struct Delegator {
    pub root_handler: RootHandler,
    pub exfil_handler: ExfilHandler,
    pub message_handler: MessageHandler,
}
impl Delegator {
    pub fn new(root_zone: &str) -> Self {
        return Delegator { 
            root_handler: RootHandler::new( LowerName::from_str(root_zone).unwrap()),
            exfil_handler: ExfilHandler::new(Delegator::construct_subdomain(root_zone, "exfil")), 
            message_handler: MessageHandler::new(Delegator::construct_subdomain(root_zone, "message")),
        }
    }

    // e.g. "example.com" + "exfil" -> "exfil.example.com"
    pub fn construct_subdomain(root_zone: &str, subdomain: &str) -> LowerName {
        let mut subdomain = subdomain.to_string();
        subdomain.push_str(".");
        subdomain.push_str(root_zone);
        return LowerName::from_str(&subdomain).unwrap();
    }

    pub async fn handle_request<R: ResponseHandler>(&self, req: &Request, res: R) -> Result<ResponseInfo, Error> {
        let name = req.query().name();
        match name {
            name if self.exfil_handler.get_zone().zone_of(name) => self.exfil_handler.handle_request(req, res).await,
            name if self.message_handler.get_zone().zone_of(name) => self.message_handler.handle_request(req, res).await,
            name if self.root_handler.get_zone().zone_of(name) => self.root_handler.handle_request(req, res).await,
            name => Err(Error::InvalidZone(name.clone()))
        }
    }
}
