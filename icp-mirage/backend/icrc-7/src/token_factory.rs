use candid::Principal;
use factory_helper::create_new_cansister_with_wasm_impl;
use ic_cdk::caller;
use std::cell::RefCell;
use std::collections::HashMap;

const MANAGER_CANISTER_PRINCIPAL: &str = "be2us-64aaa-aaaaa-qaabq-cai";

pub struct NFTFactory {
    collections: HashMap<String, Principal>, // Source chain collection address to ICP collection principal
}

impl NFTFactory {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
        }
    }

    // Store new NFT canister instance
    pub fn store_nft_collection(&mut self, src_collection_addr: String, canister_id: Principal) {
        self.collections.insert(src_collection_addr, canister_id);
    }

    // Get NFT canister instance
    pub fn get_nft_collection(&mut self, src_chain_contract_addr: String) -> Option<Principal> {
        self.collections.get(&src_chain_contract_addr).cloned()
    }

    // Get all collections
    pub fn get_collections(&self) -> HashMap<String, Principal> {
        self.collections.clone()
    }
}

thread_local! {
    pub static FACTORY: RefCell<NFTFactory> = RefCell::new(NFTFactory::new());
}

// Async update function to get or create a new NFT collection
#[ic_cdk_macros::update(guard = "is_allowed_caller")]
pub async fn get_or_create_nft_collection(
    src_chain_contract_addr: String,
) -> Result<Principal, String> {
    ic_cdk::print(format!(
        "Source chain contract address: {} {}",
        src_chain_contract_addr,
        ic_cdk::api::id()
    ));
    // If a collection canister already exists, return its principal
    let maybe_collection_id = FACTORY.with(|factory| {
        factory
            .borrow_mut()
            .get_nft_collection(src_chain_contract_addr.clone())
    });
    if let Some(collection_id) = maybe_collection_id {
        return Ok(collection_id);
    }
    // Otherwise, create a new collection canister and return its principal
    let result = create_new_cansister_with_wasm_impl().await;
    match result {
        Ok(canister_id) => {
            FACTORY.with(|factory| {
                factory
                    .borrow_mut()
                    .store_nft_collection(src_chain_contract_addr.clone(), canister_id)
            });
            return Ok(canister_id);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// Async update function to create a new NFT collection
#[ic_cdk_macros::query]
pub async fn get_nft_collection(src_chain_contract_addr: String) -> Option<Principal> {
    FACTORY.with(|factory| {
        factory
            .borrow_mut()
            .get_nft_collection(src_chain_contract_addr)
    })
}

// Query function to get the collections
#[ic_cdk_macros::query]
pub fn get_all_collections() -> HashMap<String, Principal> {
    FACTORY.with(|factory| factory.borrow().get_collections())
}

mod factory_helper {
    use candid::Principal;
    use ic_cdk::api::management_canister::main::{
        create_canister, install_code, CanisterInstallMode, CanisterSettings,
        CreateCanisterArgument, InstallCodeArgument,
    };

    pub const CREATE_CANISTER_CYCLES: u128 = 1_500_000_000_000; // 1.5T
    const ICRC7_WASM: &[u8] = include_bytes!("../../../wasm_files/icrc7.wasm");

    async fn create_new_canister() -> Result<Principal, String> {
        let create_result = create_canister(
            CreateCanisterArgument {
                settings: Some(CanisterSettings {
                    controllers: None,
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                    reserved_cycles_limit: None,
                    log_visibility: None,
                    wasm_memory_limit: None,
                }),
            },
            CREATE_CANISTER_CYCLES,
        )
        .await;

        match create_result {
            Ok(v) => {
                let canister_id = v.0.canister_id;
                return Ok(canister_id);
            }
            Err(e) => {
                let error = format!("ERROR - {:?}, {}", e.0, e.1);
                return Err(error);
            }
        }
    }

    async fn install_wasm(canister: String) -> Result<String, String> {
        let canister_id: Principal;
        match Principal::from_text(canister) {
            Ok(v) => canister_id = v,
            Err(e) => {
                let error = format!(
                    "ERROR - Could not convert text to Principal (install_wasm) - {}",
                    e
                );
                return Err(error);
            }
        }
        let mut argument: Vec<u8> = Vec::new();

        let arg = InstallCodeArgument {
            mode: CanisterInstallMode::Install,
            canister_id: canister_id,
            wasm_module: ICRC7_WASM.to_vec().clone(),
            arg: argument,
        };

        let result = install_code(arg).await;
        match result {
            Ok(_v) => return Ok(String::from("Wasm Installed")),
            Err(e) => {
                let error = format!("ERROR (install_wasm) - {:?}, {}", e.0, e.1);
                return Err(error);
            }
        }
    }

    pub async fn create_new_cansister_with_wasm_impl() -> Result<Principal, String> {
        let created_canister: Principal;

        let create_result = create_new_canister().await;
        match create_result {
            Ok(canister_id) => {
                let callres = install_wasm(canister_id.to_text()).await;
                match callres {
                    Err(e) => {
                        let error = format!("ERROR - Could not install WASM (SuperIndex) - {}", e);
                        return Err(error);
                    }
                    _ => {}
                }
                created_canister = canister_id.clone();
            }
            Err(e) => {
                let error = format!("ERROR - Could not install WASM (New Canister IMPL) - {}", e);
                return Err(error);
            }
        }

        return Ok(created_canister);
    }
}

/// Check if the caller is a custodian.
fn is_allowed_caller() -> Result<(), String> {
    let caller = &caller();
    if Principal::from_text(MANAGER_CANISTER_PRINCIPAL).unwrap() == *caller {
        Ok(())
    } else {
        Err("Only Manager canister can call this method.".to_string())
    }
}
