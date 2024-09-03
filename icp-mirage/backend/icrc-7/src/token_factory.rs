use crate::icrc7::{MintArgs, NFTContract, TransferArgs, TransferError, TransferResult};
use candid::{CandidType, Nat, Principal};
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
    contracts: Vec<NFTContract>,
}

impl NFTFactory {
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
            contracts: Vec::new(),
        }
    }

    // Create a new NFT contract instance
    pub fn create_nft_contract(&mut self, name: String, symbol: String) -> Principal {
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

        contract_address
    }

    // Function to mint a token in a specific NFT contract, delegating to ICRC7
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

    // Function to transfer tokens in a specific NFT contract, delegating to ICRC7
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

    // Function to burn a token in a specific NFT contract, delegating to ICRC7
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

// Async update function to create a new NFT collection
#[ic_cdk_macros::update]
pub async fn create_nft_collection(name: String, symbol: String) -> Principal {
    FACTORY.with(|factory| factory.borrow_mut().create_nft_contract(name, symbol))
}

// Async update function to mint a token in a specific NFT contract
#[ic_cdk_macros::update]
pub async fn mint_token_in_collection(
    collection_index: usize,
    mint_args: MintArgs,
) -> Result<Nat, TransferError> {
    FACTORY.with(|factory| {
        factory
            .borrow_mut()
            .mint_token_in_contract(collection_index, mint_args.clone())
    })
}

// Async update function to transfer tokens in a specific NFT contract
#[ic_cdk_macros::update]
pub async fn transfer_tokens_in_collection(
    collection_index: usize,
    transfer_args: Vec<TransferArgs>,
) -> Vec<Option<TransferResult>> {
    FACTORY.with(|factory| {
        factory
            .borrow_mut()
            .transfer_tokens_in_contract(collection_index, transfer_args.clone())
    })
}

// Async update function to burn a token in a specific NFT contract
#[ic_cdk_macros::update]
pub async fn burn_token_in_collection(
    collection_index: usize,
    token_id: Nat,
) -> Result<Nat, TransferError> {
    FACTORY.with(|factory| {
        factory
            .borrow_mut()
            .burn_token_in_contract(collection_index, token_id.clone())
    })
}

// Query function to get the collections
#[ic_cdk_macros::query]
pub fn get_all_collections() -> Vec<NFTCollection> {
    FACTORY.with(|factory| factory.borrow().get_collections())
}
