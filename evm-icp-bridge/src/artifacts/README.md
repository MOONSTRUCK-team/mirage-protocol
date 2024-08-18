### Id hash generation

Id = keccak(abi.encodePacked(nonce,srcChainId,destChainId,contract,tokenId))

`nonce` is a increment number that is used to avoid hash collision. It is incremented by 1 for each message.

`srcChainId` is the chain id of the source chain.

`destChainId` is the chain id of the destination chain.

`contract` is the contract address on the source chain.

`tokenId` is the id of the token that is locked.

Destination chain should have a mapping from contract address to the contract address on the destination chain.