
use candid::{Nat, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_cdk::call;
use ic_cdk_macros::{query, update};
use icrc7::types::{Account, TokenId, TransferArgs, TransferError};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static VAULT: RefCell<NFTVault> = RefCell::new(NFTVault::new());
}


pub struct NFTVault {
    
    pub vault: HashMap<Account, Vec<TokenId>>,
}

impl NFTVault {
    pub fn new() -> Self {
        Self {
            vault: HashMap::new(),
        }
    }

    // Function to store NFTs into the vault
    pub fn deposit(&mut self, account: Account, token_ids: Vec<TokenId>) {
        let account_tokens = self.vault.entry(account).or_insert_with(Vec::new);
        account_tokens.extend(token_ids);
    }

    // Function to withdraw NFTs from the vault
    pub fn withdraw(&mut self, account: Account, token_ids: Vec<TokenId>) -> Result<(), String> {
        if let Some(account_tokens) = self.vault.get_mut(&account) {
            for token_id in &token_ids {
                if !account_tokens.contains(token_id) {
                    return Err(format!("Token ID {} not found in vault for the account.", token_id));
                }
            }
            // Remove the token IDs from the vault
            account_tokens.retain(|token_id| !token_ids.contains(token_id));
            Ok(())
        } else {
            Err("Account has no tokens stored in the vault.".to_string())
        }
    }

    // Function to retrieve the stored NFTs for an account
    pub fn get_stored_nfts(&self, account: Account) -> Vec<TokenId> {
        self.vault.get(&account).cloned().unwrap_or_default()
    }
}

// Update function to deposit NFTs into the vault
#[update]
pub async fn deposit_nft(token_ids: Vec<TokenId>) -> Result<(), String> {
    let caller = ic_cdk::caller(); // The user depositing the NFTs
    let vault_principal = ic_cdk::api::id(); // The vault canister's own principal

    let transfer_args = TransferArgs {
        from: Account {
            owner: caller,
            subaccount: None,
        },
        to: Account {
            owner: vault_principal,
            subaccount: None,
        },
        token_ids: token_ids.clone(),
    };

    let icrc7_principal = Principal::from_text("icrc7-canister-principal-id").unwrap();

    let result: Result<(Vec<Option<Result<Nat, TransferError>>>,), (RejectionCode, String)> =
        call(icrc7_principal, "transfer_tokens", (vec![transfer_args],)).await;

    match result {
        Ok(_) => {
            // Store the NFTs in the vault
            VAULT.with(|vault| vault.borrow_mut().deposit(Account { owner: caller, subaccount: None }, token_ids));
            Ok(())
        }
        Err(e) => Err(format!("Failed to transfer NFTs: {:?}", e)),
    }
}

// Update function to withdraw NFTs from the vault
#[update]
pub async fn withdraw_nft(token_ids: Vec<TokenId>) -> Result<(), String> {
    let caller = ic_cdk::caller(); // The user withdrawing the NFTs
    let vault_principal = ic_cdk::api::id(); // The vault canister's own principal

    let account = Account {
        owner: caller,
        subaccount: None,
    };

    // Check if the NFTs exist in the vault
    VAULT.with(|vault| vault.borrow_mut().withdraw(account.clone(), token_ids.clone()))?;

    let transfer_args = TransferArgs {
        from: Account {
            owner: vault_principal,
            subaccount: None,
        },
        to: account.clone(),
        token_ids: token_ids.clone(),
    };

    let icrc7_principal = Principal::from_text("icrc7-canister-principal-id").unwrap();

    let result: Result<(Vec<Option<Result<Nat, TransferError>>>,), (RejectionCode, String)> =
        call(icrc7_principal, "transfer_tokens", (vec![transfer_args],)).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to transfer NFTs: {:?}", e)),
    }
}

// Query function to retrieve the stored NFTs for an account
#[query]
pub fn get_nfts_for_account() -> Vec<TokenId> {
    let caller = ic_cdk::caller(); // The user making the query

    let account = Account {
        owner: caller,
        subaccount: None,
    };

    VAULT.with(|vault| vault.borrow().get_stored_nfts(account))
}

// Function to communicate with the manager canister
#[update]
pub async fn report_to_manager(token_id: Nat) -> Result<(), String> {
    let manager_principal = Principal::from_text("manager-canister-principal-id").unwrap();

    let result: Result<(String,), (RejectionCode, String)> = 
        call(manager_principal, "notify_nft_action", (token_id,)).await;

    match result {
        Ok((response,)) => Ok(response),
        Err((code, msg)) => Err(format!("Call to manager failed: {:?} - {:?}", code, msg)),
    }
}

