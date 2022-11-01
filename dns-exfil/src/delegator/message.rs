use trust_dns_server::server::{Request,ResponseHandler,ResponseInfo};
use trust_dns_server::client::rr::LowerName;
use trust_dns_server::proto::op::{Header};
use trust_dns_server::proto::rr::{RData, Record};
use trust_dns_server::authority::{MessageResponseBuilder};
use std::net::IpAddr;
use std::str::FromStr;
use std::collections::HashMap;

use crate::delegator::Handler;
use crate::delegator::BaseHandler; // required for default trait impl
use crate::error::Error;

pub struct MessageHandler {
    pub zone: LowerName,
    inflight_requests: HashMap<String, MessageQueue>,
}

struct MessageQueue {
    pub queue: Vec<String>,
    pub num_messages: u32,
}

pub struct Start {
    pub id: String,
    pub num_messages: u32,
}

pub struct Data {
    pub id: String,
    pub message_count: u32,
    pub data: String,
}

pub struct End {
    pub id: String,
}

pub enum Message {
    Start(Start),
    Data(Data),
    End(End),
}


impl MessageHandler {
    pub fn new(zone: LowerName) -> Self {
        return MessageHandler{
            zone,
            inflight_requests: HashMap::new()
        };
    }

    fn parse_message(&self, query_name: &LowerName) -> Result<Message, Error> {
        let new_query_name = query_name.to_string();
        let new_query_name = new_query_name.trim_end_matches(&self.zone.to_string());
        let new_query_name = new_query_name.trim_end_matches(".");
        let new_query_name = new_query_name.split(".").collect::<Vec<&str>>();
        let message_type = new_query_name[0];
        match message_type {
            "start" => {
                if new_query_name.len() != 3 {
                    return Err(Error::Error());
                }
                let id = new_query_name[1].to_owned();
                let num_messages = new_query_name[2].parse::<u32>();
                match num_messages {
                    Ok(num_messages) => Ok(Message::Start(Start{id, num_messages})),
                    Err(_) => Err(Error::Error()),
                }
            },
            "data" => {
                if new_query_name.len() != 4 {
                    return Err(Error::Error());
                }
                let id = new_query_name[1].to_owned();
                let message_count = new_query_name[2].parse::<u32>();
                match message_count {
                    Ok(message_count) => Ok(Message::Data(Data{id, message_count, data: new_query_name[3].to_owned()})),
                    Err(_) => Err(Error::Error()),
                }
            },
            "end" => {
                if new_query_name.len() != 2 {
                    return Err(Error::Error());
                }
                let id = new_query_name[1].to_owned();
                Ok(Message::End(End{id}))
                },
            _ => Err(Error::MessageRouteInvalidFormat(query_name.to_owned())),
        }
    }


    fn handle_start(&self, start: Start) -> () {
        todo!();
    }
    fn handle_data(&self, data: Data) -> () {
        todo!();
    }
    fn handle_end(&self, end: End) -> () {
        todo!();
    }
}


// needs to be parallelized at the end...

#[async_trait::async_trait]
impl Handler for MessageHandler {
    async fn handle_request<R: ResponseHandler>(&self, req: &Request, res: R) -> Result<ResponseInfo, Error> {
        let query_name = req.query().name();
        let message = self.parse_message(query_name)?;
        match message {
            Message::Start(start) => self.handle_start(start),
            Message::Data(data) => self.handle_data(data),
            Message::End(end) => self.handle_end(end),
        };
        return MessageHandler::build_basic_response(req, res).await;
    }


    fn get_zone(&self) -> &LowerName {
        return &self.zone;
    }
}