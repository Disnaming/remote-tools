pub mod exfil;
pub mod root;

use crate::error::Error;
use crate::delegator::exfil::ExfilHandler;
use crate::delegator::root::RootHandler;

use trust_dns_server::client::rr::LowerName;
use trust_dns_server::server::{Request,ResponseHandler,ResponseInfo};
use std::str::FromStr;

#[async_trait::async_trait]
pub trait Handler {
    async fn handle_request<R: ResponseHandler>(req: &Request, mut res: R) -> Result<ResponseInfo, Error>;
}

#[derive(Clone, Debug)]
pub struct Delegator {
    pub root_zone: LowerName,
    pub exfil_zone: LowerName,
}
// implements the delegator
impl Delegator {
    pub fn new(root_zone: &str) -> Self {
        return Delegator{
            root_zone: LowerName::from_str(root_zone).unwrap(),
            exfil_zone: Delegator::construct_subdomain(root_zone, "exfil"),
        };
    }

    pub fn construct_subdomain(root_zone: &str, subdomain: &str) -> LowerName {
        let mut subdomain = subdomain.to_string();
        subdomain.push_str(".");
        subdomain.push_str(root_zone);
        return LowerName::from_str(&subdomain).unwrap();
    }

    pub async fn handle_request<R: ResponseHandler>(&self, req: &Request, res: R) -> Result<ResponseInfo, Error> {
        let name = req.query().name();
        match name {
            name if self.exfil_zone.zone_of(name) => ExfilHandler::handle_request(req, res).await,
            name if self.root_zone.zone_of(name) => RootHandler::handle_request(req, res).await,
            name => Err(Error::InvalidZone(name.clone()))
        }
    }
}
