#!/usr/bin/env bash

## Teardown
trap 'pkill bun index.ts && pkill anvil && dfx stop' EXIT

PARENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

## Navigate to project root
cd "$PARENT_PATH"
cd ..

## ICP setup
sh ./icp/scripts/deploy.sh

## EVM setup
cd "$PARENT_PATH"
cd ..
cd evm
anvil &
sleep 5
forge script script/Deploy.s.sol --broadcast --fork-url http://localhost:8545

## Bridge setup
cd "$PARENT_PATH"
cd ..
cd bridge
bun index.ts &
sleep 5

cd "$PARENT_PATH"
cd ..

## Mint on ICP
DEPLOYER_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
NFT_COLLECTION=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
TOKEN_ID=1
DST_CHAIN=2
DST_ACCOUNT="2vxsx-fae"
NFT_VAULT=0x5fbdb2315678afecb367f032d93f642f64180aa3
BRIDGE_MEDIATOR=0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0
MANAGER=0xcf7ed3acca5a467e9e704c703e8d87f634fb0fc9

cast send $NFT_COLLECTION "approve(address,uint256)" $NFT_VAULT 1 --private-key $DEPLOYER_PRIVATE_KEY

cast send $MANAGER "deposit(address,uint256,uint256,string)" $NFT_COLLECTION $TOKEN_ID $DST_CHAIN $DST_ACCOUNT --private-key $DEPLOYER_PRIVATE_KEY

## Wait for message to be executed on ICP
sleep 20

## Burn on ICP
cd icp
DST_COLLECTION_ADDRESS=$(echo $(dfx canister call token_factory get_nft_collection "(\"${NFT_COLLECTION}\")") | sed 's/.*principal "\(.*\)")/\1/')
echo $DST_COLLECTION_ADDRESS
dfx canister call token_factory get_nft_collection "(\"${NFT_COLLECTION}\")"
dfx canister call manager token_burn \
'(
  record {
    canister_id = '"principal \"${DST_COLLECTION_ADDRESS}\""' : principal;
    token_id = '"${TOKEN_ID}"' : nat;
  },
)'
cd "$PARENT_PATH"

## Wait for message to be executed on EVM
sleep 20