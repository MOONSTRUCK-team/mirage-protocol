use candid::Nat;
use candid::Principal;
use icrc_7::icrc7::{Account, MintArgs, TransferArgs, TransferResult, ICRC7};
use icrc_7::token::NFTContract;
use std::collections::HashMap;

fn create_test_contract(max_supply: u64) -> NFTContract {
    NFTContract {
        name: "My NFT Collection".to_string(),
        symbol: "MNC".to_string(),
        description: Some("A test NFT collection".to_string()),
        image: Some("https://example.com/image.png".to_string()),
        total_supply: Nat::from(0u64),
        max_supply: Some(Nat::from(max_supply)),
        royalties: Some(150),
        royalty_recipient: None,
        collection_metadata: vec![],
        balances: HashMap::new(),
        metadata: HashMap::new(),
    }
}

#[test]
fn test_icrc7_name() {
    let contract = create_test_contract(1000);
    assert_eq!(contract.icrc7_name(), "My NFT Collection".to_string());
}

#[test]
fn test_minting_nft() {
    let mut contract = create_test_contract(1000);

    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };

    let result = contract.icrc7_mint(mint_args);
    assert!(result.is_ok());
    assert_eq!(contract.total_supply, Nat::from(1u64));
}

#[test]
fn test_minting_beyond_max_supply() {
    let mut contract = create_test_contract(1); // max_supply is 1

    let mint_args_1 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };

    // Mint the first token, which should succeed
    let result_1 = contract.icrc7_mint(mint_args_1);
    assert!(result_1.is_ok());

    let mint_args_2 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(2u64),
        metadata: vec![],
    };

    // Attempt to mint a second token, which should fail due to max supply
    let result_2 = contract.icrc7_mint(mint_args_2);
    assert!(result_2.is_err());
}

#[test]
fn test_transfer_without_ownership() {
    let mut contract = create_test_contract(1000);

    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };

    contract.icrc7_mint(mint_args).unwrap();

    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(), // Wrong owner
            subaccount: None,
        },
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert!(matches!(transfer_result[0], Some(TransferResult::Err(_))));
}

#[test]
fn test_burning_non_existent_token() {
    let mut contract = create_test_contract(1000);

    let result = contract.icrc7_burn(Nat::from(1u64));
    assert!(result.is_err());
}

#[test]
fn test_mint_and_transfer_multiple_nfts() {
    let mut contract = create_test_contract(1000);

    // Mint two NFTs
    let mint_args_1 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    let mint_args_2 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(2u64),
        metadata: vec![],
    };

    contract.icrc7_mint(mint_args_1).unwrap();
    contract.icrc7_mint(mint_args_2).unwrap();

    // Check that the initial balance is correct
    let initial_balance = contract.icrc7_balance_of(Account {
        owner: Principal::anonymous(),
        subaccount: None,
    });
    assert_eq!(initial_balance, Nat::from(2u64));

    // Transfer both NFTs to another account
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64), Nat::from(2u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert!(matches!(transfer_result[0], Some(TransferResult::Ok(_))));

    // Check that the recipient now owns the two tokens
    let recipient_balance = contract.icrc7_balance_of(Account {
        owner: Principal::from_text("aaaaa-aa").unwrap(),
        subaccount: None,
    });
    assert_eq!(recipient_balance, Nat::from(2u64));

    // Check that the original owner no longer owns any tokens
    let sender_balance = contract.icrc7_balance_of(Account {
        owner: Principal::anonymous(),
        subaccount: None,
    });
    assert_eq!(sender_balance, Nat::from(0u64));
}

#[test]
fn test_token_ownership_after_transfer() {
    let mut contract = create_test_contract(1000);

    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };

    contract.icrc7_mint(mint_args).unwrap();

    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert!(matches!(transfer_result[0], Some(TransferResult::Ok(_))));

    // Ensure the new owner now owns the token
    let new_owner = contract.icrc7_owner_of(Nat::from(1u64));
    assert_eq!(new_owner.owner, Principal::from_text("aaaaa-aa").unwrap());

    // Ensure the previous owner no longer owns the token
    let previous_owner_balance = contract.icrc7_balance_of(Account {
        owner: Principal::anonymous(),
        subaccount: None,
    });
    assert_eq!(previous_owner_balance, Nat::from(0u64));
}

#[test]
fn test_re_minting_same_token_id() {
    let mut contract = create_test_contract(1000);

    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };

    // Mint the first token, which should succeed
    let result_1 = contract.icrc7_mint(mint_args.clone());
    assert!(result_1.is_ok());

    // Attempt to mint the same token again, which should fail
    let result_2 = contract.icrc7_mint(mint_args);
    assert!(result_2.is_err());
}

#[test]
fn test_burning_token_after_transfer() {
    let mut contract = create_test_contract(1000);

    // Mint an NFT
    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args).unwrap();

    // Transfer the NFT to another account
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);

    // Check if the transfer succeeded
    if let Some(Some(TransferResult::Ok(_))) = transfer_result.get(0) {
        // Transfer succeeded, now burn the token by the new owner
        let burn_result = contract.icrc7_burn(Nat::from(1u64));
        assert!(burn_result.is_ok());
        assert_eq!(contract.total_supply, Nat::from(0u64));
    } else {
        panic!("Transfer failed");
    }
}

#[test]
fn test_batch_transfer_with_partial_failure() {
    let mut contract = create_test_contract(1000);

    // Mint two NFTs
    let mint_args_1 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    let mint_args_2 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(2u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args_1).unwrap();
    contract.icrc7_mint(mint_args_2).unwrap();

    // Transfer one valid token and one invalid token (token ID 3 does not exist)
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64), Nat::from(3u64)], // One valid, one invalid
        memo: None,
        created_at_time: None,
        is_atomic: Some(false),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);

    // Check the result of the first transfer (valid)
    assert!(matches!(transfer_result[0], Some(TransferResult::Ok(_))));

    // Check the result of the second transfer (invalid)
    assert!(matches!(transfer_result[1], Some(TransferResult::Err(_))));
}

#[test]
fn test_atomic_batch_transfer() {
    let mut contract = create_test_contract(1000);

    // Mint two NFTs
    let mint_args_1 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    let mint_args_2 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(2u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args_1).unwrap();
    contract.icrc7_mint(mint_args_2).unwrap();

    // Transfer one valid token and one invalid token (token ID 3 does not exist)
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64), Nat::from(3u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };
    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);

    // Since this is atomic, all transfers should fail
    assert!(matches!(transfer_result[0], Some(TransferResult::Err(_))));
}

#[test]
fn test_empty_metadata_mint() {
    let mut contract = create_test_contract(1000);

    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![], // Empty metadata
    };

    let result = contract.icrc7_mint(mint_args);
    assert!(result.is_ok());

    // Check that the token was minted successfully with empty metadata
    let token_metadata = contract.icrc7_metadata(Nat::from(1u64));
    assert!(token_metadata.is_empty());
}

#[test]
fn test_max_supply_reached_in_batch_mint() {
    let mut contract = create_test_contract(3); // Max supply of 3

    let mint_args_1 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    let mint_args_2 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(2u64),
        metadata: vec![],
    };
    let mint_args_3 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(3u64),
        metadata: vec![],
    };
    let mint_args_4 = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(4u64),
        metadata: vec![],
    };

    contract.icrc7_mint(mint_args_1).unwrap();
    contract.icrc7_mint(mint_args_2).unwrap();
    contract.icrc7_mint(mint_args_3).unwrap();

    // This should fail because we have reached the max supply
    let result = contract.icrc7_mint(mint_args_4);
    assert!(result.is_err());
}

#[test]
fn test_transferring_burned_token() {
    let mut contract = create_test_contract(1000);

    // Mint a token and then burn it
    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args).unwrap();
    contract.icrc7_burn(Nat::from(1u64)).unwrap();

    // Attempt to transfer the burned token
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(false),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert!(matches!(transfer_result[0], Some(TransferResult::Err(_))));
}

#[test]
fn test_transferring_to_self() {
    let mut contract = create_test_contract(1000);

    // Mint a token
    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args).unwrap();

    // Attempt to transfer the token to yourself
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64)],
        memo: None,
        created_at_time: None,
        is_atomic: Some(true),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert!(matches!(transfer_result[0], Some(TransferResult::Ok(_))));
}

#[test]
fn test_duplicate_token_ids_in_batch_transfer() {
    let mut contract = create_test_contract(1000);

    // Mint a token
    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args).unwrap();

    // Attempt to transfer the same token twice in one batch
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64), Nat::from(1u64)], // Duplicate token ID
        memo: None,
        created_at_time: None,
        is_atomic: Some(false),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert_eq!(transfer_result.len(), 2); // Both tokens should be processed separately
    assert!(matches!(transfer_result[0], Some(TransferResult::Ok(_))));
    assert!(matches!(transfer_result[1], Some(TransferResult::Err(_)))); // The second transfer should fail
}

#[test]
fn test_large_memo_and_future_created_at_time() {
    let mut contract = create_test_contract(1000);

    // Mint a token
    let mint_args = MintArgs {
        to: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        token_id: Nat::from(1u64),
        metadata: vec![],
    };
    contract.icrc7_mint(mint_args).unwrap();

    // Attempt to transfer with a large memo and future created_at_time
    let transfer_args = TransferArgs {
        spender_subaccount: None,
        from: Account {
            owner: Principal::anonymous(),
            subaccount: None,
        },
        to: Account {
            owner: Principal::from_text("aaaaa-aa").unwrap(),
            subaccount: None,
        },
        token_ids: vec![Nat::from(1u64)],
        memo: Some(vec![0; 100000]),             // Large memo
        created_at_time: Some(9999999999999999), // Far future time
        is_atomic: Some(true),
    };

    let transfer_result = contract.icrc7_transfer(vec![transfer_args]);
    assert!(matches!(transfer_result[0], Some(TransferResult::Ok(_))));
}
