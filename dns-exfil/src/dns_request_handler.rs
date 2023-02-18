use log::error;
use trust_dns_server::proto::op::ResponseCode;

use trust_dns_server::proto::op::Header;

use trust_dns_server::server::RequestHandler;

use trust_dns_server::proto::op::MessageType;

use trust_dns_server::proto::op::OpCode;

use crate::error::Error;

use trust_dns_server::server::ResponseInfo;

use trust_dns_server::server::Request;

use trust_dns_server::server::ResponseHandler;

use crate::delegator::Delegator;

pub struct DNSRequestHandler {
    pub(crate) delegator: Delegator,
}

impl DNSRequestHandler {
    pub fn create_request_handler(domain: &str) -> Self {
        return DNSRequestHandler {
            delegator: Delegator::new(domain),
        };
    }

    pub(crate) async fn parse_request<R: ResponseHandler>(
        &self,
        req: &Request,
        res: R,
    ) -> Result<ResponseInfo, Error> {
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
impl RequestHandler for DNSRequestHandler {
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
