use candid::{CandidType, Nat};
use ic_cdk::init;
use ic_cdk_macros::{query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use types::{
    Account, MetadataEntry, MintArgs, TokenId, TransferArgs, TransferError, TransferResult,
};

mod types;

// Global state for the NFT contract
thread_local! {
    pub static CONTRACT: RefCell<Option<NFTContract>> = RefCell::new(None);
}

// NFT Contract definition
pub struct NFTContract {
    pub name: String,
    pub symbol: String,
    pub total_supply: Nat,
    pub balances: HashMap<Account, Vec<TokenId>>,
    pub metadata: HashMap<TokenId, Vec<MetadataEntry>>,
}

impl NFTContract {
    // Initialize the contract
    pub fn new(name: String, symbol: String) -> Self {
        Self {
            name,
            symbol,
            total_supply: Nat::from(0u64),
            balances: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    // Mint tokens with metadata validation
    pub fn mint(&mut self, mint_args: MintArgs) -> Result<Nat, TransferError> {
        // Check if token already exists
        if self.metadata.contains_key(&mint_args.token_id) {
            return Err(TransferError::TokenAlreadyExists {
                token_id: mint_args.token_id.clone(),
            });
        }

        // Ensure that metadata is valid (example validation)
        if mint_args.metadata.is_empty() {
            return Err(TransferError::InvalidMetadata {
                token_id: mint_args.token_id.clone(),
            });
        }

        // Add the token to the balances of the recipient
        self.balances
            .entry(mint_args.to.clone())
            .or_insert_with(Vec::new)
            .push(mint_args.token_id.clone());

        // Add metadata
        self.metadata
            .insert(mint_args.token_id.clone(), mint_args.metadata.clone());

        // Increase the total supply
        self.total_supply += Nat::from(1u64);

        Ok(mint_args.token_id)
    }

    // Transfer tokens
    pub fn transfer(&mut self, transfers: Vec<TransferArgs>) -> Vec<Option<TransferResult>> {
        let mut results = Vec::new();

        for transfer in transfers.into_iter() {
            let mut all_transfers_successful = true;
            let mut tokens_to_transfer = Vec::new();

            if let Some(sender_tokens) = self.balances.get_mut(&transfer.from) {
                for token_id in &transfer.token_ids {
                    if sender_tokens.contains(token_id) {
                        tokens_to_transfer.push(token_id.clone());
                    } else {
                        results.push(Some(TransferResult::Err(
                            TransferError::InsufficientBalance {
                                token_id: token_id.clone(),
                            },
                        )));
                        all_transfers_successful = false;
                    }
                }

                if all_transfers_successful {
                    for token_id in &tokens_to_transfer {
                        sender_tokens.retain(|id| id != token_id);
                    }

                    let recipient_tokens = self
                        .balances
                        .entry(transfer.to.clone())
                        .or_insert_with(Vec::new);
                    recipient_tokens.extend(tokens_to_transfer.clone());

                    results.push(Some(TransferResult::Ok(Nat::from(
                        tokens_to_transfer.len(),
                    ))));
                }
            } else {
                results.push(Some(TransferResult::Err(TransferError::Unauthorized {
                    token_ids: transfer.token_ids.clone(),
                })));
            }
        }

        results
    }

    // Burn tokens
    pub fn burn(&mut self, token_id: TokenId) -> Result<Nat, TransferError> {
        if let Some((_account, tokens)) = self
            .balances
            .iter_mut()
            .find(|(_, tokens)| tokens.contains(&token_id))
        {
            tokens.retain(|id| id != &token_id);
            self.metadata.remove(&token_id);
            self.total_supply -= Nat::from(1u64);
            Ok(token_id)
        } else {
            Err(TransferError::TokenNotFound {
                token_id: token_id.clone(),
            })
        }
    }
}

// Initialize contract with dynamic values
#[init]
fn init(name: String, symbol: String) {
    CONTRACT.with(|contract| {
        *contract.borrow_mut() = Some(NFTContract::new(name, symbol));
    })
}

// Free functions for canister update operations

#[update]
async fn mint_token(mint_args: MintArgs) -> Result<Nat, TransferError> {
    CONTRACT.with(|contract| {
        let mut contract_ref = contract.borrow_mut();
        if let Some(ref mut nft_contract) = *contract_ref {
            nft_contract.mint(mint_args)
        } else {
            Err(TransferError::Unauthorized {
                token_ids: vec![mint_args.token_id.clone()],
            })
        }
    })
}

#[update]
async fn transfer_tokens(transfers: Vec<TransferArgs>) -> Vec<Option<TransferResult>> {
    CONTRACT.with(|contract| {
        let mut contract_ref = contract.borrow_mut();
        if let Some(ref mut nft_contract) = *contract_ref {
            nft_contract.transfer(transfers)
        } else {
            transfers
                .into_iter()
                .map(|transfer| {
                    Some(TransferResult::Err(TransferError::Unauthorized {
                        token_ids: transfer.token_ids.clone(),
                    }))
                })
                .collect()
        }
    })
}

#[update]
async fn burn_token(token_id: TokenId) -> Result<Nat, TransferError> {
    CONTRACT.with(|contract| {
        let mut contract_ref = contract.borrow_mut();
        if let Some(ref mut nft_contract) = *contract_ref {
            nft_contract.burn(token_id)
        } else {
            Err(TransferError::TokenNotFound {
                token_id: token_id.clone(),
            })
        }
    })
}

// Query functions

// Get NFT collection name
#[query]
fn icrc7_name() -> String {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .map_or(String::from(""), |nft_contract| nft_contract.name.clone())
    })
}

// Get NFT symbol name
#[query]
fn icrc7_symbol() -> String {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .map_or(String::from(""), |nft_contract| nft_contract.symbol.clone())
    })
}

// Get token metadata by token ID
#[query]
fn get_token_metadata(token_id: TokenId) -> Option<Vec<MetadataEntry>> {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .and_then(|nft_contract| nft_contract.metadata.get(&token_id).cloned())
    })
}

// Get balance of an account
#[query]
fn get_balance(account: Account) -> Vec<TokenId> {
    CONTRACT.with(|contract| {
        contract.borrow().as_ref().map_or(vec![], |nft_contract| {
            nft_contract
                .balances
                .get(&account)
                .cloned()
                .unwrap_or_default()
        })
    })
}

// Get total supply of tokens
#[query]
fn get_total_supply() -> Nat {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .map_or(Nat::from(0u64), |nft_contract| {
                nft_contract.total_supply.clone()
            })
    })
}

// Check if an account owns a specific token
#[query]
fn is_owner(account: Account, token_id: TokenId) -> bool {
    CONTRACT.with(|contract| {
        contract.borrow().as_ref().map_or(false, |nft_contract| {
            nft_contract
                .balances
                .get(&account)
                .map_or(false, |tokens| tokens.contains(&token_id))
        })
    })
}

// Get all token IDs owned by an account
#[query]
fn get_tokens_of(account: Account) -> Vec<TokenId> {
    CONTRACT.with(|contract| {
        contract.borrow().as_ref().map_or(vec![], |nft_contract| {
            nft_contract
                .balances
                .get(&account)
                .cloned()
                .unwrap_or_default()
        })
    })
}

fn main() {}
