// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "../src/interfaces/IManager.sol";
import "../src/interfaces/INFTVault.sol";
import { CallerNotBridgeMediator } from "../src/ManagerErrors.sol";
import { MintTokenMessageSent, OnTokenBurnedCallback } from "../src/ManagerEvents.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract Manager is IManager {
    INFTVault private immutable _nftVault;
    address private immutable _bridgeMediator;

    modifier onlyBridgeMediator() {
        if (msg.sender != address(_bridgeMediator)) revert CallerNotBridgeMediator();
        _;
    }

    constructor(address nftVault_, address bridgeMediator_) {
        _nftVault = INFTVault(nftVault_);
        _bridgeMediator = bridgeMediator_;
    }

    function deposit(IERC721 collection, uint256 tokenId) external {
        _nftVault.deposit(collection, tokenId, msg.sender);

        _sendMintTokenMessage(collection, tokenId, msg.sender);
    }

    function onTokenBurned(IERC721 collection, uint256 tokenId) external {
        _nftVault.release(collection, tokenId);

        emit OnTokenBurnedCallback(address(collection), tokenId);
    }

    function _sendMintTokenMessage(IERC721 collection, uint256 tokenId, address owner) private {
        // TODO: Prepare message data

        // TODO: Send message

        emit MintTokenMessageSent(address(collection), tokenId, owner);
    }
}
