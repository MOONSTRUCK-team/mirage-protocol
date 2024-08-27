use crate::bridge_common_layer::{notify_external_service, Message};
pub use crate::icrc7::{
    Account, MintArgs, NFTContract, TransferArgs, TransferError, TransferResult,
};
use candid::{CandidType, Nat, Principal};
use num_traits::cast::ToPrimitive;
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
    contracts: Vec<NFTContract>, // Store the actual contracts
}

impl NFTFactory {
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
            contracts: Vec::new(),
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

        // Create a new NFT contract instance
        let nft_contract = NFTContract::new(name.clone(), symbol.clone());

        // Add collection metadata
        self.collections.push(NFTCollection {
            name: name.clone(),
            symbol: symbol.clone(),
            contract_address,
        });

        // Store the contract instance
        self.contracts.push(nft_contract);

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

    // Function to mint a token in a specific NFT contract
    pub fn mint_token_in_contract(
        &mut self,
        collection_index: usize,
        mint_args: MintArgs,
    ) -> Result<Nat, TransferError> {
        if let Some(contract) = self.contracts.get_mut(collection_index) {
            contract.mint(mint_args)
        } else {
            Err(TransferError::Unauthorized {
                token_ids: vec![mint_args.token_id],
            })
        }
    }

    // Function to transfer tokens in a specific NFT contract
    pub fn transfer_tokens_in_contract(
        &mut self,
        collection_index: usize,
        transfer_args: Vec<TransferArgs>,
    ) -> Vec<Option<TransferResult>> {
        if let Some(contract) = self.contracts.get_mut(collection_index) {
            contract.transfer(transfer_args)
        } else {
            vec![Some(TransferResult::Err(TransferError::Unauthorized {
                token_ids: vec![],
            }))]
        }
    }

    // Function to burn a token in a specific NFT contract
    pub fn burn_token_in_contract(
        &mut self,
        collection_index: usize,
        token_id: Nat,
    ) -> Result<Nat, TransferError> {
        if let Some(contract) = self.contracts.get_mut(collection_index) {
            contract.burn(token_id)
        } else {
            Err(TransferError::TokenNotFound { token_id })
        }
    }

    // Get all collections
    pub fn get_collections(&self) -> Vec<NFTCollection> {
        self.collections.clone()
    }
}

thread_local! {
    pub static FACTORY: RefCell<NFTFactory> = RefCell::new(NFTFactory::new());
}

// Async update function to create a new NFT collection and notify external service
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

    // Notify the external service asynchronously
    match notify_external_service(msg).await {
        Ok(_) => Ok(principal),
        Err(err) => Err(format!("Failed to notify external service: {:?}", err)),
    }
}

// Async update function to mint a token in a specific NFT contract and notify external service
#[ic_cdk_macros::update]
pub async fn mint_token_in_collection(
    collection_index: usize,
    mint_args: MintArgs,
    id: String,         // Message ID
    nonce: u64,         // Nonce value
    src_chain_id: u64,  // Source chain ID
    dest_chain_id: u64, // Destination chain ID
) -> Result<Nat, String> {
    FACTORY.with(|factory| {
        let result = factory
            .borrow_mut()
            .mint_token_in_contract(collection_index, mint_args.clone());

        match result {
            Ok(token_id) => {
                // Notify external service asynchronously
                let msg = Message {
                    id,
                    nonce,
                    op_type: 1, // Mint operation
                    src_chain_id,
                    dest_chain_id,
                    dest_address: mint_args.to.owner.to_string(),
                    contract_address: ic_cdk::api::id().to_string(),
                    token_id: token_id.0.to_u64().unwrap_or_default(),
                };

                ic_cdk::spawn(async move {
                    let _ = notify_external_service(msg).await;
                });

                Ok(token_id)
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    })
}

// Async update function to transfer tokens in a specific NFT contract and notify external service
#[ic_cdk_macros::update]
pub async fn transfer_tokens_in_collection(
    collection_index: usize,
    transfer_args: Vec<TransferArgs>,
    id: String,         // Message ID
    nonce: u64,         // Nonce value
    src_chain_id: u64,  // Source chain ID
    dest_chain_id: u64, // Destination chain ID
) -> Vec<Option<TransferResult>> {
    FACTORY.with(|factory| {
        let results = factory
            .borrow_mut()
            .transfer_tokens_in_contract(collection_index, transfer_args.clone());

        // Notify external service for successful transfers
        for (i, result) in results.iter().enumerate() {
            if let Some(TransferResult::Ok(_)) = result {
                let msg = Message {
                    id: format!("{}_{}", id, i),
                    nonce,
                    op_type: 2, // Transfer operation
                    src_chain_id,
                    dest_chain_id,
                    dest_address: transfer_args[i].to.owner.to_string(),
                    contract_address: ic_cdk::api::id().to_string(),
                    token_id: transfer_args[i].token_ids[0].0.to_u64().unwrap_or_default(),
                };

                ic_cdk::spawn(async move {
                    let _ = notify_external_service(msg).await;
                });
            }
        }

        results
    })
}

// Async update function to burn a token in a specific NFT contract and notify external service
#[ic_cdk_macros::update]
pub async fn burn_token_in_collection(
    collection_index: usize,
    token_id: Nat,
    id: String,         // Message ID
    nonce: u64,         // Nonce value
    src_chain_id: u64,  // Source chain ID
    dest_chain_id: u64, // Destination chain ID
) -> Result<Nat, String> {
    FACTORY.with(|factory| {
        let result = factory
            .borrow_mut()
            .burn_token_in_contract(collection_index, token_id.clone());

        match result {
            Ok(token_id) => {
                // Notify external service asynchronously
                let msg = Message {
                    id,
                    nonce,
                    op_type: 3, // Burn operation
                    src_chain_id,
                    dest_chain_id,
                    dest_address: "".to_string(), // No recipient in burn case
                    contract_address: ic_cdk::api::id().to_string(),
                    token_id: token_id.0.to_u64().unwrap_or_default(),
                };

                ic_cdk::spawn(async move {
                    let _ = notify_external_service(msg).await;
                });

                Ok(token_id)
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    })
}

// Query function to get the collections
#[ic_cdk_macros::query]
pub fn get_all_collections() -> Vec<NFTCollection> {
    FACTORY.with(|factory| factory.borrow().get_collections())
}
