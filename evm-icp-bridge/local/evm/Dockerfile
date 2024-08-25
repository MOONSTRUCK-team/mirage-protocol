# Use the latest Foundry image as the base image
FROM ghcr.io/foundry-rs/foundry
# Set the working directory
WORKDIR /usr/src/app

# Copy the rest of the application code
COPY . .

# Expose the port Anvil will run on
EXPOSE 8545

# Command to run Anvil and deploy the contract
CMD ["sh", "-c", "anvil & sleep 2 && forge create --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 src/BridgeMediator.sol:BridgeMediator"]