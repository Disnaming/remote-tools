
use trust_dns_server::client::rr::LowerName;
use trust_dns_server::proto::op::{OpCode, MessageType};

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