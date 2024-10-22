use candid::{CandidType, Nat, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::call;
use ic_cdk_macros::update;
use icrc7::types::{Account, MetadataEntry, MintArgs};
use manager::types::SourceCollectionArgs;
use serde::{Deserialize, Serialize};

const MANAGER_CANISTER_PRINCIPAL: &str = "be2us-64aaa-aaaaa-qaabq-cai";

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct Message {
    pub id: String,
    pub nonce: u64,
    pub op_type: u8,
    pub src_chain_id: u64,
    pub dest_chain_id: u64,
    pub dest_address: String,
    pub contract_address: String,
    pub collection_name: String,
    pub collection_symbol: String,
    pub token_id: u64,
    pub token_metadata: String,
}

// Function to fetch a message from the external bridge service and mint a token
#[update]
pub async fn execute_message(msg: Message) -> Result<Nat, String> {
    let src_collection_args = SourceCollectionArgs {
        address: msg.contract_address,
        name: msg.collection_name,
        symbol: msg.collection_symbol,
    };

    // Extract metadata
    let metadata_to_json_result: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&msg.token_metadata);
    let metadata_as_json;
    match metadata_to_json_result {
        Ok(value) => {
            metadata_as_json = value;
        }
        Err(err) => {
            return Err(format!("Failed to parse metadata: {}", err.to_string()));
        }
    }

    // Populate the expected metadata structure
    let mut metadata_entries: Vec<MetadataEntry> = vec![];
    for (key, value) in metadata_as_json.as_object().unwrap() {
        let metadata_entry = MetadataEntry {
            key: key.to_string(),
            value: value.as_str().unwrap().to_string(),
        };
        metadata_entries.push(metadata_entry);
    }

    // Convert the Message into MintArgs to use with the mint function
    let mint_args = MintArgs {
        to: Account {
            owner: Principal::from_text(msg.dest_address).unwrap(),
            subaccount: None,
        },
        token_id: Nat::from(msg.token_id),
        metadata: metadata_entries,
    };

    // Get NFT collection and call token mint
    let get_collection_call_result = call_token_mint(
        Principal::from_text(MANAGER_CANISTER_PRINCIPAL).unwrap(),
        src_collection_args,
        mint_args,
    )
    .await;

    match get_collection_call_result {
        Ok(token_id) => Ok(token_id),
        Err(e) => Err(e),
    }
}

async fn call_token_mint(
    canister_id: Principal,
    src_collection_args: SourceCollectionArgs,
    mint_args: MintArgs,
) -> Result<Nat, String> {
    let call_result: Result<(Result<Nat, String>,), (RejectionCode, String)> =
        call(canister_id, "token_mint", (src_collection_args, mint_args)).await;

    match call_result {
        Ok(value) => match value.0 {
            Ok(token_id) => Ok(token_id),
            Err(err) => Err(format!("Failed to mint token: {:?}", err)),
        },
        Err(err) => Err(format!("Failed mint call: {:?} - {:?}", err.0, err.1)),
    }
}

fn main() {}