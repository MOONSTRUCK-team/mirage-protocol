/// Description: Multi-Chain NFT Bridge Standard for ICP
use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

// ========== Type Definitions ==========

/// A subaccount type, represented as a 32-byte blob.
pub type Subaccount = [u8; 32];

/// Account representation: an owner (principal) and an optional subaccount.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType, PartialEq, Eq, Hash)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

/// Token ID is a natural number.
pub type TokenId = Nat;

/// Represents a flexible value for metadata purposes.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum Value {
    Blob(Vec<u8>),
    Text(String),
    Nat(Nat),
    Int(i128),
    Array(Vec<Value>),
    Map(Vec<(String, Value)>),
}

/// Metadata for tokens and collections.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct MetadataEntry {
    pub key: String,
    pub value: Value,
}

/// Mint arguments for creating a new token.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct MintArgs {
    pub to: Account,
    pub token_id: TokenId,
    pub metadata: Vec<MetadataEntry>,
}

/// Transfer arguments for performing a token transfer.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct TransferArgs {
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub token_ids: Vec<TokenId>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>, // Time as nanoseconds since UNIX epoch
    pub is_atomic: Option<bool>,      // Defaults to true
}

/// Transfer result indicating success or failure of a transfer.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum TransferResult {
    Ok(Nat), // Transaction index in the ledger
    Err(TransferError),
}

/// Errors that can occur during a transfer operation.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum TransferError {
    Unauthorized { token_ids: Vec<TokenId> },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

// ========== Metadata Variant Definition ==========

/// Defines the metadata that can be attached to NFTs and collections.
#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum Metadata {
    Nat(Nat),
    Int(i128),
    Text(String),
    Blob(Vec<u8>),
}

// ========== ICRC-7 Trait Definition ==========

/// The ICRC-7 trait defines the required methods for compliant NFTs on the Internet Computer.
pub trait ICRC7 {
    // ===== Collection-Level Methods =====

    /// Retrieves the name of the NFT collection (e.g., "My NFT Collection").
    fn icrc7_name(&self) -> String;

    /// Retrieves the symbol of the NFT collection (e.g., "MNC").
    fn icrc7_symbol(&self) -> String;

    /// Retrieves the description of the NFT collection.
    fn icrc7_description(&self) -> Option<String>;

    /// Retrieves the image URL of the NFT collection.
    fn icrc7_image(&self) -> Option<String>;

    /// Retrieves the total supply of NFTs in the collection.
    fn icrc7_total_supply(&self) -> Nat;

    /// Retrieves the maximum supply of NFTs allowed in the collection.
    fn icrc7_supply_cap(&self) -> Option<Nat>;

    /// Retrieves all collection-level metadata in one query.
    fn icrc7_collection_metadata(&self) -> Vec<MetadataEntry>;

    // ===== Royalty-Related Methods =====

    /// Retrieves the default royalty percentage in basis points.
    fn icrc7_royalties(&self) -> Option<u16>; // In basis points (e.g., 150 = 1.5%)

    /// Retrieves the default royalty recipient.
    fn icrc7_royalty_recipient(&self) -> Option<Account>;

    // ===== Token-Level Methods =====

    /// Retrieves the metadata of a specific token by its ID.
    fn icrc7_metadata(&self, token_id: TokenId) -> Vec<MetadataEntry>;

    /// Retrieves the owner of a specific token by its ID.
    fn icrc7_owner_of(&self, token_id: TokenId) -> Account;

    /// Retrieves the balance of NFTs owned by the specified account.
    fn icrc7_balance_of(&self, account: Account) -> Nat;

    /// Retrieves the list of token IDs owned by the specified account.
    fn icrc7_tokens_of(&self, account: Account) -> Vec<TokenId>;

    /// Performs a batch of token transfers.
    fn icrc7_transfer(&mut self, transfers: Vec<TransferArgs>) -> Vec<Option<TransferResult>>;

    // ===== Standard Support Methods =====

    /// Retrieves the list of standards implemented by this contract.
    fn icrc7_supported_standards(&self) -> Vec<(String, String)>;

    // ===== Minting Method =====

    /// Mints a new NFT with a specified token ID and metadata, and assigns it to an account.
    fn icrc7_mint(&mut self, mint_args: MintArgs) -> Result<Nat, TransferError>;

    // ===== Burning Functionality =====

    /// Burns a specific token by its ID, permanently removing it from the collection.
    fn icrc7_burn(&mut self, token_id: TokenId) -> Result<Nat, TransferError>;
}
