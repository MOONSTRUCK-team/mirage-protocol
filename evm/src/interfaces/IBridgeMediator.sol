// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

interface IBridgeMediator {
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

    function sendMessage(
        uint8 opType,
        uint256 destChainId,
        string memory destAddress,
        address contractAddress,
        uint256 tokenId
    )
        external;

    function executeMessage(Message calldata message) external;

    function getNonce() external view returns (uint256);
}
