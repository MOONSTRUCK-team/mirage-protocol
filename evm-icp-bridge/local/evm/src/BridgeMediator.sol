// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

contract BridgeMediator {
    uint256 private _nonce;

    struct Message {
        bytes32 id;
        uint256 nonce;
        uint8 opType;
        uint256 srcChainId;
        uint256 destChainId;
        string destAddress;
        address contractAddress;
        uint256 tokenId;
    }

    event MessageSend(bytes32 indexed id, Message message);
    event MessageExecuted(bytes32 indexed id, Message message);

    function sendMessage() public {
        Message memory message = Message({
            id: 0x0,
            nonce: ++_nonce,
            opType: 1,
            srcChainId: 1,
            destChainId: 2,
            destAddress: "0x1234567890123456789012345678901234567890",
            contractAddress: address(this),
            tokenId: 1
        });
        bytes32 messageId = keccak256(abi.encodePacked(
            message.nonce,
            message.opType,
            message.srcChainId,
            message.destChainId,
            message.destAddress,
            message.contractAddress,
            message.tokenId
        ));
        message.id = messageId;

        emit MessageSend(messageId, message);
    }

    function executeMessage(Message calldata message) external {
        emit MessageExecuted(message.id, message);
    }

    function getNonce() external view returns (uint256) {
        return _nonce;
    }
}