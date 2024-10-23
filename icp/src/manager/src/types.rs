use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct SourceCollectionArgs {
    pub address: String,
    pub name: String,
    pub symbol: String,
}
