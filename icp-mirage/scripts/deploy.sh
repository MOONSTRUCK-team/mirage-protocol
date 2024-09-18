#!/usr/bin/env bash

PARENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
ICRC7_WASM_PATH=".dfx/local/canisters/icrc7/icrc7.wasm"
WASM_FILES_PATH="wasm_files"

## Navigate to project root
cd "$PARENT_PATH"
cd ..

## Cleanup
#rm -rf .dfx target .env src/declarations "$WASM_FILES_PATH"

## Start local replica
dfx stop
dfx start --clean --background

## Create canisters
dfx canister create icrc7
dfx canister create token_factory
dfx canister create manager
dfx canister create bridge_common_layer

## Build canisters
dfx build icrc7
mkdir -p "$WASM_FILES_PATH" && cp "$ICRC7_WASM_PATH" "$WASM_FILES_PATH" ## Copy Wasm of ICRC-7 implementation so that it can be used for dynamic creation of new canisters
dfx build token_factory
dfx build manager
dfx build bridge_common_layer

## Install canisters
dfx canister install token_factory          --mode install
dfx canister install manager                --mode install
dfx canister install bridge_common_layer    --mode install