#!/usr/bin/env bash

PARENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$PARENT_PATH"

## Deploy canisters
source deploy.sh

## Navigate to project root
cd "$PARENT_PATH"
cd ..

## Setup
## Prepare source collection arguments
SRC_COLLECTION_ADDRESS="0x0000000000000000000000000000000000000000"
SRC_COLLECTION_NAME="My Super NFT"
SRC_COLLECTION_SYMBOL="MSNFT"
## Prepare mint arguments
dfx identity new owner --disable-encryption || true
OWNER_PRINCIPAL=$(dfx --identity owner identity get-principal)
OWNER_SUBACCOUNT=null
TOKEN_ID=1
METADATA_ATTR1="attr1"
METADATA_VAL1="val1"
TOKEN_METADATA="{\""${METADATA_ATTR1}\"":\""${METADATA_VAL1}\""}"
## Prepare bridge message arguments
MESSAGE_ID="1"
NONCE_ID=1
OP_TYPE=1
SRC_CHAIN_ID=1
DST_CHAIN_ID=2

## Helper variables
ACCOUNT_RECORD="record { owner = principal \"${OWNER_PRINCIPAL}\"; subaccount = ${OWNER_SUBACCOUNT}; }"

## Execute mint NFT message
echo '(*) Creating new NFT collection and minting NFT:'
dfx canister call bridge_common_layer execute_message \
'(
  record {
    id = '\""${MESSAGE_ID}\""';
    dest_chain_id = '"${DST_CHAIN_ID}"' : nat64;
    src_chain_id = '"${SRC_CHAIN_ID}"' : nat64;
    token_id = '"${TOKEN_ID}"' : nat64;
    dest_address = '\""${OWNER_PRINCIPAL}\""';
    collection_name = '\""${SRC_COLLECTION_NAME}\""';
    collection_symbol = '\""${SRC_COLLECTION_SYMBOL}\""';
    nonce = '"${NONCE_ID}"' : nat64;
    contract_address = '\""${SRC_COLLECTION_ADDRESS}\""';
    op_type = '"${OP_TYPE}"' : nat8;
    token_metadata = "{\"attr1\":\"val1\"}" : text;
  },
)'

#dfx canister call manager token_mint \
#"(
#  record {
#    address = \"${SRC_COLLECTION_ADDRESS}\";
#    name = \"${SRC_COLLECTION_NAME}\";
#    symbol = \"${SRC_COLLECTION_SYMBOL}\";
#  },  
#  record {
#    to = $ACCOUNT_RECORD;
#    token_id = $TOKEN_ID : nat;
#    metadata = vec { record { key = \"${METADATA_ATTR1}\"; value = \"${METADATA_VAL1}\" } };
#  }
#)"

DST_COLLECTION_ADDRESS=$(echo $(dfx canister call token_factory get_nft_collection "(\"${SRC_COLLECTION_ADDRESS}\")") | sed 's/.*principal "\(.*\)")/\1/')

echo '(*) Collection name:'
dfx canister call $DST_COLLECTION_ADDRESS icrc7_name

echo '(*) Collection symbol:'
dfx canister call $DST_COLLECTION_ADDRESS icrc7_symbol

echo '(*) Total NFTs in existence:'
dfx canister call $DST_COLLECTION_ADDRESS get_total_supply

echo "(*) Token balance for $OWNER_PRINCIPAL:"
dfx canister call $DST_COLLECTION_ADDRESS get_balance \
"(
   $ACCOUNT_RECORD
)"

echo "(*) Tokens of $OWNER_PRINCIPAL:"
dfx canister call $DST_COLLECTION_ADDRESS get_tokens_of \
"(
   $ACCOUNT_RECORD
)"

echo "(*) Is $OWNER_PRINCIPAL owner of NFT $TOKEN_ID:"
dfx canister call $DST_COLLECTION_ADDRESS is_owner \
"(
   $ACCOUNT_RECORD,
   $TOKEN_ID : nat
)"

echo "(*) NFT $TOKEN_ID metadata:"
dfx canister call $DST_COLLECTION_ADDRESS get_token_metadata \
"(
   $TOKEN_ID : nat
)"

## Teardown
dfx identity remove owner;
