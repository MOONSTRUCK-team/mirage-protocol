// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

event MintTokenMessageSent(address indexed collection, uint256 indexed tokenId, address indexed owner);

event OnTokenBurnedCallback(address indexed collection, uint256 indexed tokenId, address indexed owner);
