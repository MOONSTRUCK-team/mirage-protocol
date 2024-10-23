// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "../src/interfaces/INFTVault.sol";
import { TokenAlreadyDeposited, TokenNotDeposited } from "../src/NFTVaultErrors.sol";
import { TokenDeposited, TokenReleased } from "../src/NFTVaultEvents.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract NFTVault is INFTVault {
    struct VaultRecord {
        address owner;
        bool isActive;
    }

    mapping(address collection => mapping(uint256 tokenId => VaultRecord record)) public records;

    function deposit(IERC721 collection, uint256 tokenId, address owner) external {
        VaultRecord storage record = records[address(collection)][tokenId];

        // Checks
        if (record.isActive) {
            revert TokenAlreadyDeposited(address(collection), tokenId);
        }

        // Effects
        record.isActive = true;
        record.owner = owner;
        emit TokenDeposited(address(collection), tokenId, owner);

        // Interactions
        collection.transferFrom(owner, address(this), tokenId);
        assert(collection.ownerOf(tokenId) == address(this));
    }

    function release(IERC721 collection, uint256 tokenId) external {
        VaultRecord storage record = records[address(collection)][tokenId];
        address owner = record.owner;

        // Checks
        if (!record.isActive) {
            revert TokenNotDeposited(address(collection), tokenId);
        }

        // Effects
        delete records[address(collection)][tokenId];
        emit TokenReleased(address(collection), tokenId, owner);

        // Interactions
        collection.transferFrom(address(this), owner, tokenId);
        assert(collection.ownerOf(tokenId) == address(owner));
    }
}
