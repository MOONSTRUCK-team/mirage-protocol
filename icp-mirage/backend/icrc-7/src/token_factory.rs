use crate::icrc7::{with_contract, Account, MetadataEntry, NFTContract};
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

// ========== Token Factory Contract ==========

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct NFTCollection {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub max_supply: Option<Nat>,
    pub royalties: Option<u16>,
    pub royalty_recipient: Option<Account>,
    pub collection_metadata: Vec<MetadataEntry>,
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

    // Function to create a new NFT contract instance
    pub fn create_nft_contract(
        &mut self,
        name: String,
        symbol: String,
        description: Option<String>,
        image: Option<String>,
        max_supply: Option<Nat>,
        royalties: Option<u16>,
        royalty_recipient: Option<Account>,
        collection_metadata: Vec<MetadataEntry>,
    ) -> Principal {
        let contract_address = ic_cdk::api::id();

        let new_collection = NFTCollection {
            name: name.clone(),
            symbol: symbol.clone(),
            description: description.clone(),
            image: image.clone(),
            max_supply: max_supply.clone(),
            royalties: royalties.clone(),
            royalty_recipient: royalty_recipient.clone(),
            collection_metadata: collection_metadata.clone(),
            contract_address,
        };

        self.collections.push(new_collection);

        // Initialize the NFT contract state using the accessor function from icrc7
        with_contract(|contract| {
            *contract = Some(NFTContract::new(
                name,
                symbol,
                description,
                image,
                max_supply,
                royalties,
                royalty_recipient,
                collection_metadata,
            ));
        });

        contract_address
    }

    // Retrieve the list of deployed NFT collections
    pub fn get_collections(&self) -> Vec<NFTCollection> {
        self.collections.clone()
    }
}

// ========== Global State ==========

thread_local! {
    pub static FACTORY: RefCell<NFTFactory> = RefCell::new(NFTFactory::new());
}

// ========== Update Functions ==========

#[ic_cdk_macros::update]
pub fn create_nft_collection(
    name: String,
    symbol: String,
    description: Option<String>,
    image: Option<String>,
    max_supply: Option<Nat>,
    royalties: Option<u16>,
    royalty_recipient: Option<Account>,
    collection_metadata: Vec<MetadataEntry>,
) -> Principal {
    FACTORY.with(|factory| {
        factory.borrow_mut().create_nft_contract(
            name,
            symbol,
            description,
            image,
            max_supply,
            royalties,
            royalty_recipient,
            collection_metadata,
        )
    })
}

// ========== Query Functions ==========

#[ic_cdk_macros::query]
pub fn get_nft_collections() -> Vec<NFTCollection> {
    FACTORY.with(|factory| factory.borrow().get_collections())
}
