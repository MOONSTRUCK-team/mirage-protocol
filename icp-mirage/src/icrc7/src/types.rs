use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

pub type Subaccount = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

pub type TokenId = Nat;

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub struct MintArgs {
    pub to: Account,
    pub token_id: TokenId,
    pub metadata: Vec<MetadataEntry>,
}

// Metadata structure with PartialEq derived
#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct TransferArgs {
    pub from: Account,
    pub to: Account,
    pub token_ids: Vec<TokenId>,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransferResult {
    Ok(Nat),
    Err(TransferError),
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum TransferError {
    Unauthorized { token_ids: Vec<TokenId> },
    TokenAlreadyExists { token_id: TokenId },
    TokenNotFound { token_id: TokenId },
    InsufficientBalance { token_id: TokenId },
    InvalidMetadata { token_id: TokenId },
}
