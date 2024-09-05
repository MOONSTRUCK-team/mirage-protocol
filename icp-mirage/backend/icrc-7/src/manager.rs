use crate::icrc7::{MintArgs, TransferError, CONTRACT};
use candid::Nat;
use ic_cdk_macros::update;

pub struct Manager;

impl Manager {
    // Function to handle minting by invoking the mint method of the icrc7 contract directly
    pub fn mint(mint_args: MintArgs) -> Result<Nat, TransferError> {
        // Access the global CONTRACT using thread_local storage, ensuring safe access
        CONTRACT.with(|contract| {
            // Check if the contract has been initialized and is available
            if let Some(ref mut nft_contract) = *contract.borrow_mut() {
                // Call the mint method of the NFT contract, passing in the MintArgs
                nft_contract.mint(mint_args)
            } else {
                // If the contract is not initialized, return an Unauthorized TransferError
                Err(TransferError::Unauthorized {
                    token_ids: vec![mint_args.token_id.clone()], // Specify the token ID that failed
                })
            }
        })
    }
}

#[update]
pub async fn token_mint(mint_args: MintArgs) -> Result<Nat, String> {
    // Call the mint function in the Manager struct to handle the minting logic
    match Manager::mint(mint_args) {
        // If the minting is successful, return the minted token's ID
        Ok(token_id) => Ok(token_id),
        // If minting fails, return an error message with details
        Err(err) => Err(format!("Failed to mint token: {:?}", err)),
    }
}
