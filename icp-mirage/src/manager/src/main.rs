use candid::{Nat, Principal};
use ic_cdk::api::call::{call, RejectionCode};
use ic_cdk_macros::update;
use icrc7::types::{MintArgs, TransferError};
use types::SourceCollectionArgs;

mod types;

const TOKEN_FACTORY_CANISTER_PRINCIPAL: &str = "bkyz2-fmaaa-aaaaa-qaaaq-cai";

#[update]
pub async fn token_mint(
    src_collection_args: SourceCollectionArgs,
    mint_args: MintArgs,
) -> Result<Nat, String> {
    // Get NFT collection
    let get_collection_call_result = call_get_collection(
        Principal::from_text(TOKEN_FACTORY_CANISTER_PRINCIPAL).unwrap(),
        src_collection_args.address,
        src_collection_args.name,
        src_collection_args.symbol,
    )
    .await;
    match get_collection_call_result {
        Ok(collection_id) => {
            //let call_result: Result<(Nat,), (RejectionCode, String)> =
            //    call(collection_id, "get_total_supply", ()).await;
            //match call_result {
            //    Ok(value) => return Ok(value.0),
            //    Err(e) => return Err(e.1),
            //}
            // Mint token
            let mint_call_result = call_mint(collection_id, mint_args).await;
            match mint_call_result {
                Ok(token_id) => return Ok(token_id),
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    }
}

async fn call_get_collection(
    canister_id: Principal,
    src_chain_contract_addr: String,
    name: String,
    symbol: String,
) -> Result<Principal, String> {
    let call_result: Result<(Result<Principal, String>,), (RejectionCode, String)> = call(
        canister_id,
        "get_or_create_nft_collection",
        (src_chain_contract_addr, name, symbol),
    )
    .await;
    match call_result {
        Ok(value) => match value.0 {
            Ok(collection_id) => {
                return Ok(collection_id);
            }
            Err(err) => return Err(format!("Failed to get or create NFT collection: {:?}", err)),
        },
        Err(err) => {
            return Err(format!(
                "Failed get or create NFT collection call: {:?} - {:?}",
                err.0, err.1
            ))
        }
    }
}

async fn call_mint(canister_id: Principal, mint_args: MintArgs) -> Result<Nat, String> {
    let call_result: Result<(Result<Nat, TransferError>,), (RejectionCode, String)> =
        call(canister_id, "mint_token", (mint_args,)).await;
    match call_result {
        Ok(value) => match value.0 {
            Ok(token_id) => {
                return Ok(token_id);
            }
            Err(err) => return Err(format!("Failed to mint token: {:?}", err)),
        },
        Err(err) => return Err(format!("Failed mint call: {:?} - {:?}", err.0, err.1)),
    }
}

fn main() {}
