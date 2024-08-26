use crate::bridge_common_layer::{notify_external_service, Message};
pub use crate::icrc7::{Account, MintArgs, NFTContract};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct NFTCollection {
    pub name: String,
    pub symbol: String,
    pub contract_address: Principal,
}

pub struct NFTFactory {
    collections: Vec<NFTCollection>,
}

impl NFTFactory {
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
        }
    }

    // Create a new NFT contract instance and notify external service
    pub fn create_nft_contract(
        &mut self,
        name: String,
        symbol: String,
        id: String,         // Message ID, passed as a parameter
        nonce: u64,         // Nonce value, passed as a parameter or generated
        src_chain_id: u64,  // Source chain ID, passed as a parameter
        dest_chain_id: u64, // Destination chain ID, passed as a parameter
        token_id: u64,      // Token ID, passed as a parameter
    ) -> (Principal, Message) {
        let contract_address = ic_cdk::api::id();

        self.collections.push(NFTCollection {
            name: name.clone(),
            symbol: symbol.clone(),
            contract_address,
        });

        // Dynamically construct the message
        let msg = Message {
            id,
            nonce,
            op_type: 1, // `1` represents a mint operation
            src_chain_id,
            dest_chain_id,
            dest_address: contract_address.to_string(),
            contract_address: contract_address.to_string(),
            token_id,
        };

        (contract_address, msg)
    }

    pub fn get_collections(&self) -> Vec<NFTCollection> {
        self.collections.clone()
    }
}

thread_local! {
    pub static FACTORY: RefCell<NFTFactory> = RefCell::new(NFTFactory::new());
}

// Async update function to create a new NFT collection
#[ic_cdk_macros::update]
pub async fn create_nft_collection(
    name: String,
    symbol: String,
    id: String,         // Message ID
    nonce: u64,         // Nonce value
    src_chain_id: u64,  // Source chain ID
    dest_chain_id: u64, // Destination chain ID
    token_id: u64,      // Token ID
) -> Result<Principal, String> {
    // Create the contract and message inside the factory closure
    let (principal, msg) = FACTORY.with(|factory| {
        factory.borrow_mut().create_nft_contract(
            name,
            symbol,
            id,
            nonce,
            src_chain_id,
            dest_chain_id,
            token_id,
        )
    });

    // Notify the external service asynchronously outside of the closure
    match notify_external_service(msg).await {
        Ok(_) => Ok(principal),
        Err(err) => Err(format!("Failed to notify external service: {:?}", err)),
    }
}

// Query function to get the collections
#[ic_cdk_macros::query]
pub fn get_all_collections() -> Vec<NFTCollection> {
    FACTORY.with(|factory| factory.borrow().get_collections())
}
