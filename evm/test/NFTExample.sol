// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract NFTExample is ERC721 {
    constructor() ERC721("NFT Example", "NFTE") {
        _mint(msg.sender, 1);
    }
}
