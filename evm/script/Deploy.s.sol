// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25 <0.9.0;

import { Manager } from "../src/Manager.sol";
import { NFTVault } from "../src/NFTVault.sol";
import { BridgeMediator } from "../src/BridgeMediator.sol";
import { NFTExample } from "../test/NFTExample.sol";

import { BaseScript } from "./Base.s.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract Deploy is BaseScript {
    function run() public broadcast returns (Manager manager) {
        NFTVault nftVault = new NFTVault();
        NFTExample collection = new NFTExample();
        BridgeMediator bridgeMediator = new BridgeMediator(address(0));
        manager = new Manager(address(nftVault), address(bridgeMediator));
        bridgeMediator.setManager(address(manager));
    }
}
