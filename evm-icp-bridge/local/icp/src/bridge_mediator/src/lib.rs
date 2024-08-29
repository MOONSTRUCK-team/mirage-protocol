use std::string::String;
use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize)]
struct Message {
    id: String, // Unique identifier for the message
    nonce: u64, // Unique message nonce
    op_type: u8, // Operation type
    src_chain_id: u64, // Source chain id
    dest_chain_id: u64, // Destination chain id
    dest_address: String, // Destination address
    contract_address: String, // Contract address
    token_id: u64, // Token id
    token_metadata: String, // Token metadata
}

#[derive(CandidType, Debug)]
enum ExecuteError {
    MethodNotImplemented,
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[ic_cdk::update]
fn execute_message(msg: Message) {
    ic_cdk::print(format!("Message received with id: {}", msg.id));
}

#[ic_cdk::update]
fn send_message() -> Result<(), ExecuteError> {
    return Err(ExecuteError::MethodNotImplemented);
}