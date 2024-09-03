use crate::bridge_common_layer::{notify_external_service, Message};
use crate::icrc7::{Account, MetadataEntry, MintArgs, TransferError, CONTRACT};
use candid::{Nat, Principal};
use ic_cdk_macros::update;
use num_traits::cast::ToPrimitive;

pub struct Manager;

impl Manager {
    // Function to handle minting via message received from bridge_communication_layer
    pub async fn handle_mint_message(msg: Message) -> Result<Nat, TransferError> {
        let metadata: Vec<MetadataEntry> = vec![];

        let mint_args = MintArgs {
            to: Manager::principal_to_account(msg.dest_address.clone()),
            token_id: Nat::from(msg.token_id),
            metadata,
        };

        // Access the global CONTRACT directly from icrc7
        let mint_result = CONTRACT.with(|contract| {
            if let Some(ref mut nft_contract) = *contract.borrow_mut() {
                nft_contract.mint(mint_args.clone())
            } else {
                Err(TransferError::Unauthorized {
                    token_ids: vec![mint_args.token_id.clone()],
                })
            }
        });

        if let Ok(ref _token_id) = mint_result {
            // Construct the message to send to the external bridge
            let new_msg = Message {
                id: msg.id,                       // Use the ID provided in the original message
                nonce: msg.nonce,                 // Use the nonce provided in the original message
                op_type: 1,                       // Assuming 1 represents a 'mint' operation
                src_chain_id: 142857, // ICP Chain ID (since the mint is happening on ICP)
                dest_chain_id: msg.dest_chain_id, // Destination chain ID from the original message ( EVM Chain ID)
                dest_address: msg.dest_address.clone(),
                contract_address: msg.contract_address.clone(),
                token_id: msg.token_id,
                token_metadata: String::new(), // No metadata to send, or use serialized metadata
            };

            // Notify external service asynchronously after minting
            ic_cdk::spawn(async move {
                let _ = notify_external_service(new_msg).await;
            });
        }

        mint_result
    }

    // Function to handle burning and notifying the bridge layer
    pub async fn handle_burn(
        msg: Message,
        token_id: Nat,
        owner: Principal,
        dest_chain_id: u64,
        dest_address: String,
    ) -> Result<Nat, TransferError> {
        // Access the global CONTRACT directly from icrc7
        let burn_result = CONTRACT.with(|contract| {
            if let Some(ref mut nft_contract) = *contract.borrow_mut() {
                nft_contract.burn(token_id.clone())
            } else {
                Err(TransferError::TokenNotFound {
                    token_id: token_id.clone(),
                })
            }
        });

        if let Ok(ref _burned_token_id) = burn_result {
            // Construct the message to send to the external bridge
            let new_msg = Message {
                id: msg.id,       // Use the ID provided in the original message
                nonce: msg.nonce, // Use the nonce provided in the original message
                op_type: 2,       // Assuming 2 represents a 'burn' operation
                src_chain_id: 1,  // EVM Chain ID (since the burn is happening on EVM)
                dest_chain_id,    // ICP Chain ID ( to synchronize this action)
                dest_address: dest_address.clone(),
                contract_address: owner.to_text(),
                token_id: token_id.clone().0.to_u64().ok_or_else(|| {
                    TransferError::InvalidMetadata {
                        token_id: token_id.clone(),
                    }
                })?,
                token_metadata: String::new(), // No metadata to send, or use serialized metadata
            };

            // Notify external service asynchronously after burning
            ic_cdk::spawn(async move {
                let _ = notify_external_service(new_msg).await;
            });
        }

        burn_result
    }

    // Helper function to convert Principal to Account
    fn principal_to_account(principal: String) -> Account {
        Account {
            owner: Principal::from_text(principal).unwrap(),
            subaccount: None,
        }
    }
}

// Update functions exposed to the canister

#[update]
async fn mint(msg: Message) -> Result<Nat, String> {
    match Manager::handle_mint_message(msg).await {
        Ok(token_id) => Ok(token_id),
        Err(err) => Err(format!("Failed to mint token: {:?}", err)),
    }
}

#[update]
async fn burn(
    msg: Message,
    token_id: Nat,
    owner: Principal,
    dest_chain_id: u64,
    dest_address: String,
) -> Result<Nat, String> {
    match Manager::handle_burn(msg, token_id, owner, dest_chain_id, dest_address).await {
        Ok(token_id) => Ok(token_id),
        Err(err) => Err(format!("Failed to burn token: {:?}", err)),
    }
}
