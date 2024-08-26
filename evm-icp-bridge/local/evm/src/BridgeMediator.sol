// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

contract BridgeMediator {
    uint256 private _nonce;

    struct Message {
        uint256 nonce;
        uint8 opType;
        uint256 srcChainId;
        uint256 destChainId;
        string destAddress;
        address contractAddress;
        uint256 tokenId;
    }

    event MessageSend(bytes32 indexed id, Message message);

    function sendMessage() public {
        Message memory message = Message(
            ++_nonce,
            1,
            1,
            2,
            "0x1234567890",
            address(this),
            1
        );
        bytes32 messageId = keccak256(abi.encodePacked(
            message.nonce,
            message.opType,
            message.srcChainId,
            message.destChainId,
            message.destAddress,
            message.contractAddress, message.
            tokenId
        ));

        emit MessageSend(messageId, message);
    }

    function getNonce() public view returns (uint256) {
        return _nonce;
    }
}