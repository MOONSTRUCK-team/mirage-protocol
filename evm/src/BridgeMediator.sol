// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "../src/interfaces/IBridgeMediator.sol";
import "../src/interfaces/IManager.sol";

contract BridgeMediator is IBridgeMediator {
    IManager private _manager;
    uint256 private _nonce;

    event MessageSend(bytes32 indexed id, Message message);
    event MessageExecuted(bytes32 indexed id, Message message);

    constructor(address manager_) {
        _manager = IManager(manager_);
    }

    function sendMessage(
        uint8 opType,
        uint256 destChainId,
        string memory destAddress,
        address contractAddress,
        uint256 tokenId
    )
        public
    {
        Message memory message;
        message.nonce = ++_nonce;
        message.opType = opType;
        message.srcChainId = block.chainid;
        message.destChainId = destChainId;
        message.destAddress = destAddress;
        message.contractAddress = contractAddress;
        message.tokenId = tokenId;

        bytes32 messageId = keccak256(
            abi.encodePacked(
                message.nonce,
                message.opType,
                message.srcChainId,
                message.destChainId,
                message.destAddress,
                message.contractAddress,
                message.tokenId
            )
        );
        message.id = messageId;

        emit MessageSend(messageId, message);
    }

    function executeMessage(Message calldata message) external {
        require(message.opType == 2, "BridgeMediator: Invalid operation type");
        require(message.destChainId == block.chainid, "BridgeMediator: Invalid destination chain");

        _manager.onTokenBurned(IERC721(message.contractAddress), message.tokenId);

        emit MessageExecuted(message.id, message);
    }

    function getNonce() external view returns (uint256) {
        return _nonce;
    }

    function setManager(address manager_) external {
        _manager = IManager(manager_);
    }
}
