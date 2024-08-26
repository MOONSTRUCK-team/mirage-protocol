use std::string::String;
use candid::CandidType;
use serde::Deserialize;

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
}

#[ic_cdk::update]
fn execute_message(msg: Message) {
    ic_cdk::print(format!("Message received with id: {}", msg.id));
}