// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

interface IManager {
    function deposit(IERC721 collection, uint256 tokenId) external;

    function onTokenBurned(IERC721 collection, uint256 tokenId, address owner) external;
}
