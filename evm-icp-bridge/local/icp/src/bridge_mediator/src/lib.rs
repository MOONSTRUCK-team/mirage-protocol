use std::string::String;
use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
struct Message {
    id: String,
    nonce: u64,
    src_chain_id: u64,
    dest_chain_id: u64,
    dest_address: String,
    contract_address: String,
    token_id: u64,
}

#[ic_cdk::update]
fn execute_message(msg: Message) {
    ic_cdk::print(format!("Message received with id: {}", msg.id));
}