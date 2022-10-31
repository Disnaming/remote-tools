use trust_dns_server::server::{Request,ResponseHandler,ResponseInfo};
use trust_dns_server::proto::op::{Header};
use trust_dns_server::proto::rr::{RData, Record};
use trust_dns_server::authority::{MessageResponseBuilder};
use std::net::IpAddr;

use crate::delegator::Handler;
use crate::error::Error;

pub struct RootHandler {}

#[async_trait::async_trait]
impl Handler for RootHandler {
    async fn handle_request<R: ResponseHandler>(req: &Request, mut res: R) -> Result<ResponseInfo, Error> {
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
}