use log::info;
use trust_dns_server::client::rr::LowerName;
use trust_dns_server::server::{Request, ResponseHandler, ResponseInfo};

use crate::delegator::BaseHandler; // required for default trait impl
use crate::delegator::HandlerInterface;
use crate::error::Error;

use super::ImmutableHandler;

pub struct RootHandler {
    pub zone: LowerName,
}

impl RootHandler {
    pub fn new(zone: LowerName) -> Self {
        return RootHandler { zone };
    }
}

#[async_trait::async_trait]
impl ImmutableHandler for RootHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        req: &Request,
        res: R,
    ) -> Result<ResponseInfo, Error> {
        let query_name = req.query().name().to_string();
        info!("Root query: {}", query_name);
        return RootHandler::build_basic_response(req, res).await;
    }
}

#[async_trait::async_trait]
impl HandlerInterface for RootHandler {
    fn get_zone(&self) -> &LowerName {
        return &self.zone;
    }
}
