// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "../src/interfaces/IManager.sol";
import "../src/interfaces/INFTVault.sol";
import "../src/interfaces/IBridgeMediator.sol";
import { CallerNotBridgeMediator } from "../src/ManagerErrors.sol";
import { MintTokenMessageSent, OnTokenBurnedCallback } from "../src/ManagerEvents.sol";
import { OpType } from "../src/ManagerTypes.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract Manager is IManager {
    INFTVault private immutable _nftVault;
    IBridgeMediator private immutable _bridgeMediator;

    modifier onlyBridgeMediator() {
        if (msg.sender != address(_bridgeMediator)) revert CallerNotBridgeMediator();
        _;
    }

    constructor(address nftVault_, address bridgeMediator_) {
        _nftVault = INFTVault(nftVault_);
        _bridgeMediator = IBridgeMediator(bridgeMediator_);
    }

    function deposit(IERC721 collection, uint256 tokenId, uint256 dstChainId, string memory dstAddress) external {
        _nftVault.deposit(collection, tokenId, msg.sender);

        _sendMintTokenMessage(collection, tokenId, msg.sender, dstChainId, dstAddress);
    }

    function onTokenBurned(IERC721 collection, uint256 tokenId) external onlyBridgeMediator {
        _nftVault.release(collection, tokenId);

        emit OnTokenBurnedCallback(address(collection), tokenId);
    }

    function _sendMintTokenMessage(
        IERC721 collection,
        uint256 tokenId,
        address owner,
        uint256 dstChainId,
        string memory dstAddress
    )
        private
    {
        // TODO: Prepare message data

        // TODO: Send message
        _bridgeMediator.sendMessage(uint8(OpType.Mint), dstChainId, dstAddress, address(collection), tokenId);

        emit MintTokenMessageSent(address(collection), tokenId, owner);
    }
}
