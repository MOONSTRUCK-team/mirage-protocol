use candid::{CandidType, Nat, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_cdk::call;
use ic_cdk_macros::update;
use icrc7::types::{Account, TokenId, TransferArgs, TransferError};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

// Struct to hold vault records
pub struct VaultRecord {
    pub owner: Principal,
    pub is_active: bool,
}

// Global vault state
thread_local! {
    pub static VAULT: RefCell<HashMap<(Principal, TokenId), VaultRecord>> = RefCell::new(HashMap::new());
}

// Error messages with CandidType and Serialize/Deserialize derivations
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum VaultError {
    TokenAlreadyDeposited(String),
    TransferFailed(String),
}

// Update function to deposit NFTs into the vault
#[update]
pub async fn deposit_nft(
    collection: Principal,
    token_id: TokenId,
    owner: Principal,
) -> Result<(), VaultError> {
    //  Check if the token is already deposited
    let already_deposited = VAULT.with(|vault| {
        let vault = vault.borrow();
        if let Some(record) = vault.get(&(collection, token_id.clone())) {
            if record.is_active {
                return Some(VaultError::TokenAlreadyDeposited(format!(
                    "Token ID {} from collection {} is already deposited.",
                    token_id, collection
                )));
            }
        }
        None
    });

    if let Some(error) = already_deposited {
        return Err(error);
    }

    //  Prepare transfer args to move the NFT to the vault
    let vault_principal = ic_cdk::api::id();
    let transfer_args = TransferArgs {
        from: Account {
            owner,
            subaccount: None,
        },
        to: Account {
            owner: vault_principal,
            subaccount: None,
        },
        token_ids: vec![token_id.clone()],
    };

    // Call the ICRC-7 canister to transfer the NFT to the vault
    let icrc7_principal = collection;
    let result: Result<(Vec<Option<Result<Nat, TransferError>>>,), (RejectionCode, String)> =
        call(icrc7_principal, "transfer_tokens", (vec![transfer_args],)).await;

    // Handle the transfer result
    match result {
        Ok(_) => {
            // Store the NFT in the vault
            VAULT.with(|vault| {
                vault.borrow_mut().insert(
                    (collection, token_id),
                    VaultRecord {
                        owner,
                        is_active: true,
                    },
                );
            });
            Ok(())
        }
        Err((code, msg)) => Err(VaultError::TransferFailed(format!(
            "Failed to transfer NFT: {:?} - {}",
            code, msg
        ))),
    }
}

fn main() {}
