use crate::icrc7::{MintArgs, TransferError};
use candid::Nat;
use candid::{CandidType, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::call;
use ic_cdk_macros::update;

use serde::{Deserialize, Serialize};

const MANAGER_CANISTER_PRINCIPAL: &str = "be2us-64aaa-aaaaa-qaabq-cai";

// Define the Message struct, representing a message from the external bridge
#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct Message {
    pub id: String,
    pub nonce: u64,
    pub op_type: u8,
    pub src_chain_id: u64,
    pub dest_chain_id: u64,
    pub dest_address: String,
    pub contract_address: String,
    pub token_id: u64,
    pub token_metadata: String,
}

// Function to fetch a message from the external bridge service and mint a token
#[update]
pub async fn execute_message(msg: Message) -> Result<Nat, String> {
    // Convert the Message into MintArgs to use with the mint function
    let mint_args = MintArgs {
        to: crate::icrc7::Account {
            owner: Principal::from_text(msg.dest_address).unwrap(), // Convert dest_address to Principal
            subaccount: None,                                       // No subaccount
        },
        token_id: Nat::from(msg.token_id), // Convert token_id to Nat
        metadata: vec![],                  // No metadata provided
    };

    // Get NFT collection
    let get_collection_call_result = call_token_mint(
        Principal::from_text(MANAGER_CANISTER_PRINCIPAL).unwrap(),
        mint_args,
    )
    .await;
    match get_collection_call_result {
        Ok(token_id) => return Ok(token_id),
        Err(e) => return Err(e),
    }
}

async fn call_token_mint(canister_id: Principal, mint_args: MintArgs) -> Result<Nat, String> {
    let call_result: Result<(Result<Nat, TransferError>,), (RejectionCode, String)> =
        call(canister_id, "token_mint", (mint_args,)).await;
    match call_result {
        Ok(value) => match value.0 {
            Ok(token_id) => {
                return Ok(token_id);
            }
            Err(err) => return Err(format!("Failed to mint token: {:?}", err)),
        },
        Err(err) => return Err(format!("Failed mint call: {:?} - {:?}", err.0, err.1)),
    }
}
