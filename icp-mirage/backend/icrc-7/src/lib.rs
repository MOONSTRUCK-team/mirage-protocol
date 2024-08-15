use candid::{CandidType, Deserialize, Principal};
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::collections::HashMap;
use std::collections::HashSet;
extern crate lazy_static;

// Struct for Account, which includes an owner (principal) and an optional subaccount.
#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

// Type definition for Subaccount, represented as a 32-byte array.
type Subaccount = [u8; 32];

// Enum for generic value type in accordance with ICRC-3. This allows for various data types.
#[derive(CandidType, Deserialize, Clone)]
pub enum Value {
    Blob(Vec<u8>),               // Binary data
    Text(String),                // Text data
    Nat(u64),                    // Unsigned integer
    Int(i64),                    // Signed integer
    Array(Vec<Value>),           // Array of values
    Map(HashMap<String, Value>), // Map of key-value pairs
}

// Struct for Transfer arguments, including subaccount, recipient account, token ID, memo, and creation time.
#[derive(CandidType, Deserialize, Clone)]
pub struct TransferArg {
    from_subaccount: Option<Subaccount>, // The subaccount to transfer the token from
    to: Account,                         // The recipient account
    token_id: u64,                       // The ID of the token to transfer
    memo: Option<Vec<u8>>,               // Optional memo for the transfer
    created_at_time: Option<u64>,        // Optional creation time for the transaction
}

// Enum for Transfer result, indicating success with a transaction index or an error.
#[derive(CandidType, Deserialize, Clone)]
pub enum TransferResult {
    Ok(u64),            // Successful transfer with transaction index
    Err(TransferError), // Error encountered during transfer
}

// Enum for Transfer errors, with various possible error types.
#[derive(CandidType, Deserialize, Clone)]
pub enum TransferError {
    NonExistingTokenId { token_id: u64 }, // Token ID does not exist
    InvalidRecipient { recipient: Account }, // Recipient account is invalid
    Unauthorized { caller: Principal },   // Unauthorized operation
    InsufficientBalance { balance: u64 }, // Insufficient balance for the operation
    TooOld,                               // Transaction is too old
    CreatedInFuture { ledger_time: u64 }, // Transaction created in the future
    Duplicate { duplicate_of: u64 },      // Duplicate transaction
    GenericError { error_code: u64, message: String }, // Generic error with code and message
}

// Storage for balances, total supply, and other important state information
#[derive(CandidType, Deserialize, Default, Clone)]
pub struct TokenState {
    balances: HashMap<Account, u64>,
    total_supply: u64,
    nonces: HashMap<Account, u64>, // To prevent replay attacks
    supply_cap: Option<u64>,       // Optional supply cap to limit token supply
}

// Struct for representing a block in the ledger
#[derive(CandidType, Deserialize, Clone)]
pub struct Block {
    pub btype: String,         // Block type (e.g., "7mint", "7burn")
    pub tid: u64,              // Token ID
    pub from: Option<Account>, // Optional from account (burn)
    pub to: Option<Account>,   // Optional to account (mint)
    pub meta: Option<Value>,   // Optional metadata (mint)
    pub ts: u64,               // Timestamp when block was created
}

// Storage for the ledger blocks
#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Ledger {
    pub blocks: Vec<Block>,
}

// Authorization Helper Function
lazy_static::lazy_static! {
    static ref AUTHORIZED_MINTERS: HashSet<Principal> = {
        let mut set = HashSet::new();
        set.insert(Principal::from_slice(&[/* authorized principal */])); // Replace with the actual principal
        set
    };
}

// Helper function to check if the caller is an authorized minter
fn is_authorized_minter(caller: Principal) -> bool {
    AUTHORIZED_MINTERS.contains(&caller)
}

// Initialize the token state and storage
#[init]
pub fn init() {
    storage::stable_save((TokenState::default(),)).expect("Failed to initialize storage.");
}

// Backup state before upgrading the canister
#[pre_upgrade]
fn pre_upgrade() {
    let state = storage::stable_restore::<(TokenState,)>()
        .expect("Failed to access storage before upgrade.");
    storage::stable_save(state).expect("Failed to save state before upgrade.");
}

// Restore state after upgrading the canister
#[post_upgrade]
fn post_upgrade() {
    let state =
        storage::stable_restore::<(TokenState,)>().expect("Failed to restore state after upgrade.");
    storage::stable_save(state).expect("Failed to save state after upgrade.");
}

// Initialize the token supply for the owner
#[update]
pub fn initialize_supply(initial_supply: u64) {
    let mut state = storage::stable_restore::<(TokenState,)>()
        .expect("Failed to access storage")
        .0;
    state.total_supply = initial_supply;
    let owner = ic_cdk::caller();
    let owner_account = Account {
        owner,
        subaccount: None,
    };
    state.balances.insert(owner_account, initial_supply);
    storage::stable_save((state,)).expect("Failed to save state.");
}

// Transfer tokens between accounts, with optional atomicity
#[update]
pub async fn icrc7_transfer(
    args: Vec<TransferArg>,
    atomic: Option<bool>,
) -> Result<Vec<Option<TransferResult>>, String> {
    let mut state = storage::stable_restore::<(TokenState,)>()
        .expect("Failed to access storage")
        .0;
    let mut results = vec![];

    for arg in args {
        let result = perform_transfer(arg.clone(), &mut state).await;
        results.push(result.clone());

        if atomic.unwrap_or(false) {
            if let Some(TransferResult::Err(_)) = result {
                return Err("Atomic transfer failed; rolling back.".into());
            }
        }
    }

    // Save the state after batch processing if atomic is not enabled
    storage::stable_save((state,)).expect("Failed to save state.");
    Ok(results)
}

// Perform a single token transfer
pub async fn perform_transfer(arg: TransferArg, state: &mut TokenState) -> Option<TransferResult> {
    // Check if the token ID exists
    if !state.balances.contains_key(&arg.to) {
        return Some(TransferResult::Err(TransferError::NonExistingTokenId {
            token_id: arg.token_id,
        }));
    }

    // Ensure the caller is authorized to transfer the token
    let caller = ic_cdk::caller();
    let from_account = Account {
        owner: caller,
        subaccount: arg.from_subaccount.clone(),
    };

    if !state.balances.contains_key(&from_account) {
        return Some(TransferResult::Err(TransferError::Unauthorized { caller }));
    }

    // Ensure sufficient balance
    let balance = state.balances.get(&from_account).copied().unwrap_or(0);
    if balance < 1 {
        return Some(TransferResult::Err(TransferError::InsufficientBalance {
            balance,
        }));
    }

    // Prevent replay attacks using a nonce mechanism
    let nonce = {
        let nonce_entry = state.nonces.entry(from_account.clone()).or_insert(0);
        *nonce_entry += 1;
        *nonce_entry
    };

    // Update balances (deduct from sender, add to receiver)
    state
        .balances
        .entry(from_account.clone())
        .and_modify(|e| *e -= 1);
    state
        .balances
        .entry(arg.to.clone())
        .and_modify(|e| *e += 1)
        .or_insert(1);

    // Save the state
    storage::stable_save((state.clone(),)).expect("Failed to save state.");

    // Record the transfer and return success
    Some(TransferResult::Ok(nonce))
}

// Mint new tokens for the specified account
#[update]
pub fn mint(to: Account, amount: u64, token_id: u64, meta: Option<Value>) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut state = storage::stable_restore::<(TokenState,)>()
        .expect("Failed to access storage")
        .0;

    // Check if the caller is authorized to mint tokens
    if !is_authorized_minter(caller) {
        return Err("Caller is not authorized to mint tokens".into());
    }

    // Check supply cap
    if let Some(cap) = state.supply_cap {
        if state.total_supply + amount > cap {
            return Err("Minting would exceed supply cap".into());
        }
    }

    // Increase the balance of the recipient account
    state
        .balances
        .entry(to.clone())
        .and_modify(|e| *e += amount)
        .or_insert(amount);

    // Increase the total supply
    state.total_supply += amount;

    // Save the updated state
    storage::stable_save((state.clone(),)).expect("Failed to save state.");

    Ok(())
}

// Query the total supply of tokens
#[query]
pub fn icrc7_total_supply() -> u64 {
    let state = storage::stable_restore::<(TokenState,)>()
        .expect("Failed to access storage")
        .0;
    state.total_supply
}

// Query the balance of specific accounts
#[query]
pub fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<u64> {
    let state = storage::stable_restore::<(TokenState,)>()
        .expect("Failed to access storage")
        .0;
    accounts
        .iter()
        .map(|account| *state.balances.get(account).unwrap_or(&0))
        .collect()
}

// Metadata queries

#[query]
pub fn icrc7_symbol() -> String {
    String::new() // Placeholder symbol
}

#[query]
fn icrc7_name() -> String {
    String::new() // Placeholder name
}

#[query]
pub fn icrc7_description() -> Option<String> {
    Some(String::new()) // Placeholder description
}

#[query]
pub fn icrc7_logo() -> Option<String> {
    Some("https://example.com/logo.png".to_string()) // Placeholder logo URL
}

// Additional placeholder queries for optional features

#[query]
pub fn icrc7_supply_cap() -> Option<u64> {
    None // Placeholder for supply cap
}

#[query]
pub fn icrc7_max_query_batch_size() -> Option<u64> {
    None // Placeholder for max query batch size
}

#[query]
pub fn icrc7_max_update_batch_size() -> Option<u64> {
    None // Placeholder for max update batch size
}

#[query]
fn icrc7_default_take_value() -> Option<u64> {
    None // Placeholder for default take value
}

#[query]
pub fn icrc7_max_take_value() -> Option<u64> {
    None // Placeholder for max take value
}

#[query]
pub fn icrc7_max_memo_size() -> Option<u64> {
    None // Placeholder for max memo size
}

#[query]
pub fn icrc7_atomic_batch_transfers() -> Option<bool> {
    None // Placeholder for atomic batch transfers
}

#[query]
pub fn icrc7_tx_window() -> Option<u64> {
    None // Placeholder for transaction window
}

#[query]
pub fn icrc7_permitted_drift() -> Option<u64> {
    None // Placeholder for permitted time drift
}

#[query]
fn icrc7_token_metadata(token_ids: Vec<u64>) -> Vec<Option<Vec<(String, Value)>>> {
    token_ids.into_iter().map(|_| None).collect() // Placeholder for token metadata
}

#[query]
pub fn icrc7_owner_of(token_ids: Vec<u64>) -> Vec<Option<Account>> {
    token_ids.into_iter().map(|_| None).collect() // Placeholder for owner of tokens
}

#[query]
pub fn icrc7_tokens(_prev: Option<u64>, _take: Option<u64>) -> Vec<u64> {
    vec![] // Placeholder for token IDs
}

#[query]
pub fn icrc7_tokens_of(_account: Account, _prev: Option<u64>, _take: Option<u64>) -> Vec<u64> {
    vec![] // Placeholder for token IDs owned by an account
}

// ICRC-10 Compliance Method
#[query]
pub fn icrc10_supported_standards() -> Vec<(String, String)> {
    vec![
        (
            "ICRC-7".to_string(),
            "https://github.com/dfinity/ICRC/issues/7".to_string(),
        ),
        (
            "ICRC-10".to_string(),
            "https://github.com/dfinity/ICRC/issues/10".to_string(),
        ),
    ]
}
