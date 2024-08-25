## Pre-requisites

1. Install Foundry tools

#### Local testing setup

1. Run `anvil` with default setup
2. To deploy the test contract, run the following:
    ```shell
    forge create --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 src/BridgeMediator.sol:BridgeMediator
    ```
   1. Deploys the BridgeMediator contract to the local chain
   2. The private key is the first one set up by default during `anvil` running
   3. Contract should be deployed at address `0x5FbDB2315678afecb367f032d93F642f64180aa3`
3. To trigger the `MessageSend` event, execute the following
    ```shell
    cast send 0x5FbDB2315678afecb367f032d93F642f64180aa3 0xe5aed28a --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
    ```