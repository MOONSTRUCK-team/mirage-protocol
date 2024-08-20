use candid::{Nat, Principal};
use icrc_7::icrc7::{
    Account, MetadataEntry, MintArgs, NFTContract, TransferArgs, TransferResult, Value,
};

#[test]
fn test_mint_transfer_burn() {
    // Initialize the NFT contract
    let mut contract = NFTContract::new(
        "Test NFT Collection".to_string(),
        "TNC".to_string(),
        Some("A test NFT collection".to_string()),
        Some("https://example.com/image.png".to_string()),
        Some(Nat::from(100u64)),
        Some(150),
        None,
        vec![],
    );

    // Create accounts
    let account_1 = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    let account_2 = Account {
        owner: Principal::anonymous(),
        subaccount: None,
    };

    // Mint a token for account_1
    let mint_args = MintArgs {
        to: account_1.clone(),
        token_id: Nat::from(1u64),
        metadata: vec![MetadataEntry {
            key: "name".to_string(),
            value: Value::Text("Test Token".to_string()),
        }],
    };

    let mint_result = contract.mint(mint_args.clone());
    assert!(mint_result.is_ok(), "Minting failed");
    assert_eq!(
        contract.total_supply,
        Nat::from(1u64),
        "Total supply mismatch after mint"
    );

    // Check balance of account_1
    let balance = contract.balances.get(&account_1).unwrap();
    assert_eq!(balance.len(), 1, "Balance length mismatch after mint");
    assert_eq!(
        balance[0],
        Nat::from(1u64),
        "Token ID mismatch for account_1 after mint"
    );

    // Transfer token from account_1 to account_2
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: account_1.clone(),
        to: account_2.clone(),
        token_ids: vec![Nat::from(1u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };

    let transfer_result = contract.transfer(vec![transfer_args.clone()]);
    assert_eq!(
        transfer_result[0],
        Some(TransferResult::Ok(Nat::from(1u64))),
        "Transfer result mismatch"
    );

    // Check balance of account_2
    let balance_2 = contract.balances.get(&account_2).unwrap();
    assert_eq!(balance_2.len(), 1, "Balance length mismatch after transfer");
    assert_eq!(
        balance_2[0],
        Nat::from(1u64),
        "Token ID mismatch for account_2 after transfer"
    );

    // Burn the token from account_2
    let burn_result = contract.burn(Nat::from(1u64));
    assert!(burn_result.is_ok(), "Burning failed");
    assert_eq!(
        contract.total_supply,
        Nat::from(0u64),
        "Total supply mismatch after burn"
    );

    // Create a binding for an empty Vec
    let empty_vec = Vec::new();

    // Ensure token was removed from account_2
    let balance_2_after_burn = contract.balances.get(&account_2).unwrap_or(&empty_vec);
    assert!(
        balance_2_after_burn.is_empty(),
        "Account_2 should have no tokens after burn"
    );
}
