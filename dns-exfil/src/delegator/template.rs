use trust_dns_server::client::rr::LowerName;
use trust_dns_server::server::{Request,ResponseHandler,ResponseInfo};
use std::str::FromStr;

use crate::delegator::Handler;
use crate::delegator::BaseHandler; // required for default trait impl
use crate::error::Error;

pub struct TemplateHandler {
    pub zone: LowerName
}

impl TemplateHandler {
    pub fn new(zone: LowerName) -> Self {
        return TemplateHandler{
            zone
        };
    }
}

#[async_trait::async_trait]
impl Handler for TemplateHandler {
    async fn handle_request<R: ResponseHandler>(req: &Request, res: R) -> Result<ResponseInfo, Error> {
        let query_name = req.query().name().to_string();
        info!("Root query: {}", query_name);
        return RootHandler::build_basic_response(req, res).await;
    }

    fn get_zone(&self) -> &LowerName {
        return &self.zone;
    }
}