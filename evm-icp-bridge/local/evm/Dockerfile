# Use the latest Foundry image as the base image
FROM ghcr.io/foundry-rs/foundry
# Set the working directory
WORKDIR /usr/src/app

# Copy the rest of the application code
COPY . .

# Expose the port Anvil will run on
EXPOSE 8545

# Command to run Anvil and deploy the contract
# `while true; do sleep 1; done` is required to keep the container running when the main process finishes, as Anvil running in the background will not keep the container running
CMD ["anvil --host 0.0.0.0 & sleep 2 && forge create --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 src/BridgeMediator.sol:BridgeMediator && while true; do sleep 1; done"]