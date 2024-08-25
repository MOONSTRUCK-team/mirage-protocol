## Pre-requisites

1. Install DFINITY SDK
2. Install Node.js
3. Install Rust

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.


### Generating bindings

To generate JS and TS bindings for your canisters, you can use the following command:

```bash
dfx generate
```