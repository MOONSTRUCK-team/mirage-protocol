use crate::icrc7::{
    Account, MetadataEntry, MintArgs, TransferArgs, TransferError, TransferResult, ICRC7,
};
use candid::Nat;
use std::collections::HashMap;

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
    pub balances: HashMap<Account, Vec<Nat>>, // Mapping from Account to Token IDs
    pub metadata: HashMap<Nat, Vec<MetadataEntry>>, // Token metadata
}

impl ICRC7 for NFTContract {
    fn icrc7_name(&self) -> String {
        self.name.clone()
    }

    fn icrc7_symbol(&self) -> String {
        self.symbol.clone()
    }

    fn icrc7_description(&self) -> Option<String> {
        self.description.clone()
    }

    fn icrc7_image(&self) -> Option<String> {
        self.image.clone()
    }

    fn icrc7_total_supply(&self) -> Nat {
        self.total_supply.clone()
    }

    fn icrc7_supply_cap(&self) -> Option<Nat> {
        self.max_supply.clone()
    }

    fn icrc7_collection_metadata(&self) -> Vec<MetadataEntry> {
        self.collection_metadata.clone()
    }

    fn icrc7_royalties(&self) -> Option<u16> {
        self.royalties
    }

    fn icrc7_royalty_recipient(&self) -> Option<Account> {
        self.royalty_recipient.clone()
    }

    fn icrc7_metadata(&self, token_id: Nat) -> Vec<MetadataEntry> {
        self.metadata.get(&token_id).cloned().unwrap_or_default()
    }

    fn icrc7_owner_of(&self, token_id: Nat) -> Account {
        self.balances
            .iter()
            .find(|(_, tokens)| tokens.contains(&token_id))
            .map(|(account, _)| account.clone())
            .expect("Token not found")
    }

    fn icrc7_balance_of(&self, account: Account) -> Nat {
        Nat::from(self.balances.get(&account).map_or(0, |tokens| tokens.len()))
    }

    fn icrc7_tokens_of(&self, account: Account) -> Vec<Nat> {
        self.balances.get(&account).cloned().unwrap_or_default()
    }

    fn icrc7_transfer(&mut self, transfers: Vec<TransferArgs>) -> Vec<Option<TransferResult>> {
        let mut results = Vec::new();

        for transfer in transfers.into_iter() {
            for token_id in &transfer.token_ids {
                // Ensure the sender owns the token
                if let Some(sender_tokens) = self.balances.get_mut(&transfer.from) {
                    if sender_tokens.contains(token_id) {
                        // Transfer the token
                        sender_tokens.retain(|id| id != token_id);

                        // Add the token to the recipient's balance
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

            // Handle atomic rollback
            if transfer.is_atomic.unwrap_or(true)
                && results
                    .iter()
                    .any(|res| matches!(res, Some(TransferResult::Err(_))))
            {
                // Rollback the transfers that succeeded
                for token_id in &transfer.token_ids {
                    if let Some(recipient_tokens) = self.balances.get_mut(&transfer.to) {
                        recipient_tokens.retain(|id| id != token_id);
                    }
                }

                // Return a generic error for atomic rollback and stop further processing
                return vec![Some(TransferResult::Err(TransferError::GenericError {
                    error_code: Nat::from(1u64),
                    message: "Atomic transfer failed".to_string(),
                }))];
            }
        }

        results
    }

    fn icrc7_supported_standards(&self) -> Vec<(String, String)> {
        vec![("ICRC7".to_string(), "1.0.0".to_string())]
    }

    fn icrc7_mint(&mut self, mint_args: MintArgs) -> Result<Nat, TransferError> {
        // Check if the token already exists
        if self.metadata.contains_key(&mint_args.token_id) {
            return Err(TransferError::GenericError {
                error_code: Nat::from(2u64),
                message: "Token ID already exists".to_string(),
            });
        }

        // Check if minting exceeds max supply
        if let Some(supply_cap) = &self.max_supply {
            if self.total_supply >= *supply_cap {
                return Err(TransferError::GenericError {
                    error_code: Nat::from(1u64),
                    message: "Max supply reached".to_string(),
                });
            }
        }

        // Add the token to the recipient's balance
        let tokens = self
            .balances
            .entry(mint_args.to.clone())
            .or_insert_with(Vec::new);
        tokens.push(mint_args.token_id.clone());

        // Set the metadata for the token
        self.metadata
            .insert(mint_args.token_id.clone(), mint_args.metadata.clone());

        // Increment total supply
        self.total_supply += Nat::from(1u64);

        Ok(mint_args.token_id)
    }

    fn icrc7_burn(&mut self, token_id: Nat) -> Result<Nat, TransferError> {
        // Check if the token exists
        if let Some(account) = self.balances.iter_mut().find_map(|(account, tokens)| {
            if tokens.contains(&token_id) {
                Some(account.clone())
            } else {
                None
            }
        }) {
            // Remove the token from the owner's balance
            self.balances
                .get_mut(&account)
                .unwrap()
                .retain(|id| id != &token_id);

            // Remove the metadata
            self.metadata.remove(&token_id);

            // Decrement the total supply
            self.total_supply -= Nat::from(1u64);

            Ok(token_id)
        } else {
            Err(TransferError::Unauthorized {
                token_ids: vec![token_id],
            })
        }
    }
}
