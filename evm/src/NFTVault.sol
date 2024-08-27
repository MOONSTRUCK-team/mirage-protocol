// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "../src/interfaces/INFTVault.sol";
import { TokenAlreadyDeposited, TokenNotDeposited } from "../src/NFTVaultErrors.sol";
import { TokenDeposited, TokenReleased } from "../src/NFTVaultEvents.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract NFTVault is INFTVault {
    struct VaultRecord {
        bool isActive;
    }

    mapping(address owner => mapping(address collection => mapping(uint256 tokenId => VaultRecord record))) public
        records;

    function deposit(IERC721 collection, uint256 tokenId) external {
        VaultRecord storage record = records[msg.sender][address(collection)][tokenId];

        // Checks
        if (record.isActive) {
            revert TokenAlreadyDeposited(address(collection), tokenId);
        }

        // Effects
        record.isActive = true;
        emit TokenDeposited(address(collection), tokenId, msg.sender);

        // Interactions
        collection.transferFrom(msg.sender, address(this), tokenId);
        assert(collection.ownerOf(tokenId) == address(this));
    }

    function release(IERC721 collection, uint256 tokenId) external {
        VaultRecord storage record = records[msg.sender][address(collection)][tokenId];

        // Checks
        if (!record.isActive) {
            revert TokenNotDeposited(address(collection), tokenId);
        }

        // Effects
        record.isActive = false;
        emit TokenReleased(address(collection), tokenId, msg.sender);

        // Interactions
        collection.transferFrom(address(this), msg.sender, tokenId);
        assert(collection.ownerOf(tokenId) == address(msg.sender));
    }
}
