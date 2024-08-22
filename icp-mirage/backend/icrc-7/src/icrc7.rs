use candid::{CandidType, Nat, Principal};
use ic_cdk_macros::{query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

// ========== Type Definitions ==========

pub type Subaccount = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

pub type TokenId = Nat;

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum Value {
    Blob(Vec<u8>),
    Text(String),
    Nat(Nat),
    Int(i128),
    Array(Vec<Value>),
    Map(Vec<(String, Value)>),
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct MetadataEntry {
    pub key: String,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct MintArgs {
    pub to: Account,
    pub token_id: TokenId,
    pub metadata: Vec<MetadataEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct TransferArgs {
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub token_ids: Vec<TokenId>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub is_atomic: Option<bool>,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransferResult {
    Ok(Nat),
    Err(TransferError),
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum TransferError {
    Unauthorized { token_ids: Vec<TokenId> },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

// ========== Global State ==========

thread_local! {
    static CONTRACT: RefCell<Option<NFTContract>> = RefCell::new(None);
}

// Allows thread-safe access to the `CONTRACT` state via a closure.
pub fn with_contract<F, R>(f: F) -> R
where
    F: FnOnce(&mut Option<NFTContract>) -> R,
{
    CONTRACT.with(|contract| f(&mut contract.borrow_mut()))
}

// ========== NFT Contract ==========

pub struct NFTContract {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub total_supply: Nat,
    pub max_supply: Option<Nat>,
    pub royalties: Option<u16>,
    pub royalty_recipient: Option<Account>,
    pub collection_metadata: Vec<MetadataEntry>,
    pub balances: HashMap<Account, Vec<TokenId>>,
    pub metadata: HashMap<TokenId, Vec<MetadataEntry>>,
}

impl NFTContract {
    // Dynamic constructor with user-supplied values
    pub fn new(
        name: String,
        symbol: String,
        description: Option<String>,
        image: Option<String>,
        max_supply: Option<Nat>,
        royalties: Option<u16>,
        royalty_recipient: Option<Account>,
        collection_metadata: Vec<MetadataEntry>,
    ) -> Self {
        Self {
            name,
            symbol,
            description,
            image,
            total_supply: Nat::from(0u64),
            max_supply,
            royalties,
            royalty_recipient,
            collection_metadata,
            balances: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    // Mint function
    pub fn mint(&mut self, mint_args: MintArgs) -> Result<Nat, TransferError> {
        if self.metadata.contains_key(&mint_args.token_id) {
            return Err(TransferError::GenericError {
                error_code: Nat::from(2u64),
                message: "Token ID already exists".to_string(),
            });
        }

        if let Some(supply_cap) = &self.max_supply {
            if self.total_supply >= *supply_cap {
                return Err(TransferError::GenericError {
                    error_code: Nat::from(1u64),
                    message: "Max supply reached".to_string(),
                });
            }
        }

        let tokens = self
            .balances
            .entry(mint_args.to.clone())
            .or_insert_with(Vec::new);
        tokens.push(mint_args.token_id.clone());

        self.metadata
            .insert(mint_args.token_id.clone(), mint_args.metadata.clone());

        self.total_supply += Nat::from(1u64);

        Ok(mint_args.token_id)
    }

    // Transfer function
    pub fn transfer(&mut self, transfers: Vec<TransferArgs>) -> Vec<Option<TransferResult>> {
        let mut results = Vec::new();

        for transfer in transfers.into_iter() {
            for token_id in &transfer.token_ids {
                if let Some(sender_tokens) = self.balances.get_mut(&transfer.from) {
                    if sender_tokens.contains(token_id) {
                        sender_tokens.retain(|id| id != token_id);

                        let recipient_tokens = self
                            .balances
                            .entry(transfer.to.clone())
                            .or_insert_with(Vec::new);
                        recipient_tokens.push(token_id.clone());

                        results.push(Some(TransferResult::Ok(Nat::from(1u64))));
                    } else {
                        results.push(Some(TransferResult::Err(TransferError::Unauthorized {
                            token_ids: vec![token_id.clone()],
                        })));
                    }
                } else {
                    results.push(Some(TransferResult::Err(TransferError::Unauthorized {
                        token_ids: vec![token_id.clone()],
                    })));
                }
            }

            if transfer.is_atomic.unwrap_or(true)
                && results
                    .iter()
                    .any(|res| matches!(res, Some(TransferResult::Err(_))))
            {
                for token_id in &transfer.token_ids {
                    if let Some(recipient_tokens) = self.balances.get_mut(&transfer.to) {
                        recipient_tokens.retain(|id| id != token_id);
                    }
                }

                return vec![Some(TransferResult::Err(TransferError::GenericError {
                    error_code: Nat::from(1u64),
                    message: "Atomic transfer failed".to_string(),
                }))];
            }
        }

        results
    }

    // Burn function
    pub fn burn(&mut self, token_id: TokenId) -> Result<Nat, TransferError> {
        if let Some(account) = self.balances.iter_mut().find_map(|(account, tokens)| {
            if tokens.contains(&token_id) {
                Some(account.clone())
            } else {
                None
            }
        }) {
            self.balances
                .get_mut(&account)
                .unwrap()
                .retain(|id| id != &token_id);

            self.metadata.remove(&token_id);

            self.total_supply -= Nat::from(1u64);

            Ok(token_id)
        } else {
            Err(TransferError::Unauthorized {
                token_ids: vec![token_id],
            })
        }
    }
}

// ========== Query and Update Functions ==========

// Initialize contract with dynamic values
#[update]
fn init_contract(
    name: String,
    symbol: String,
    description: Option<String>,
    image: Option<String>,
    max_supply: Option<Nat>,
    royalties: Option<u16>,
    royalty_recipient: Option<Account>,
    collection_metadata: Vec<MetadataEntry>,
) {
    CONTRACT.with(|contract| {
        *contract.borrow_mut() = Some(NFTContract::new(
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
}

// Collection-Level Methods (Queries)

#[query]
fn get_name() -> String {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().name.clone())
}

#[query]
fn get_symbol() -> String {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().symbol.clone())
}

#[query]
fn get_description() -> Option<String> {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().description.clone())
}

#[query]
fn get_image() -> Option<String> {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().image.clone())
}

#[query]
fn get_total_supply() -> Nat {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().total_supply.clone())
}

#[query]
fn get_supply_cap() -> Option<Nat> {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().max_supply.clone())
}

#[query]
fn get_collection_metadata() -> Vec<MetadataEntry> {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .unwrap()
            .collection_metadata
            .clone()
    })
}

#[query]
fn get_royalties() -> Option<u16> {
    CONTRACT.with(|contract| contract.borrow().as_ref().unwrap().royalties)
}

#[query]
fn get_royalty_recipient() -> Option<Account> {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .unwrap()
            .royalty_recipient
            .clone()
    })
}

// Token-Level Methods (Queries)

#[query]
fn get_metadata(token_id: TokenId) -> Vec<MetadataEntry> {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .unwrap()
            .metadata
            .get(&token_id)
            .cloned()
            .unwrap_or_default()
    })
}

#[query]
fn get_owner_of(token_id: TokenId) -> Account {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .unwrap()
            .balances
            .iter()
            .find(|(_, tokens)| tokens.contains(&token_id))
            .map(|(account, _)| account.clone())
            .expect("Token not found")
    })
}

#[query]
fn get_balance_of(account: Account) -> Nat {
    CONTRACT.with(|contract| {
        Nat::from(
            contract
                .borrow()
                .as_ref()
                .unwrap()
                .balances
                .get(&account)
                .map_or(0, |tokens| tokens.len()),
        )
    })
}

#[query]
fn get_tokens_of(account: Account) -> Vec<TokenId> {
    CONTRACT.with(|contract| {
        contract
            .borrow()
            .as_ref()
            .unwrap()
            .balances
            .get(&account)
            .cloned()
            .unwrap_or_default()
    })
}

#[query]
fn get_supported_standards() -> Vec<(String, String)> {
    vec![("ICRC7".to_string(), "1.0.0".to_string())]
}

// State-Modifying Methods (Updates)

#[update]
fn mint(mint_args: MintArgs) -> Result<Nat, TransferError> {
    CONTRACT.with(|contract| contract.borrow_mut().as_mut().unwrap().mint(mint_args))
}

#[update]
fn transfer(transfers: Vec<TransferArgs>) -> Vec<Option<TransferResult>> {
    CONTRACT.with(|contract| contract.borrow_mut().as_mut().unwrap().transfer(transfers))
}

#[update]
fn burn(token_id: TokenId) -> Result<Nat, TransferError> {
    CONTRACT.with(|contract| contract.borrow_mut().as_mut().unwrap().burn(token_id))
}
