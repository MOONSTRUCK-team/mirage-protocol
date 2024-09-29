// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25 <0.9.0;

import { Test } from "forge-std/src/Test.sol";

import { Manager } from "../src/Manager.sol";
import { NFTVault } from "../src/NFTVault.sol";
import { NFTExample } from "./NFTExample.sol";

import { CallerNotBridgeMediator } from "../src/ManagerErrors.sol";
import { MintTokenMessageSent, OnTokenBurnedCallback } from "../src/ManagerEvents.sol";

contract ManagerTest is Test {
    NFTVault internal nftVault;
    NFTExample internal collection;
    Manager internal manager;
    address internal bridgeMediator = makeAddr("bridgeMediator");

    function setUp() public virtual {
        nftVault = new NFTVault();
        collection = new NFTExample();
        manager = new Manager(address(nftVault), bridgeMediator);
    }

    function test_deposit_SuccessfulDeposit() external {
        // Arrange
        collection.approve(address(nftVault), 1);

        vm.expectEmit(address(manager));
        emit MintTokenMessageSent(address(collection), 1, address(this));

        // Act
        manager.deposit(collection, 1);

        // Assert
        assertEq(collection.ownerOf(1), address(nftVault));
    }

    function test_onTokenBurned_RevertIf_CallerNotBridgeMediator() external {
        // Arrange

        // Assert
        vm.expectRevert(abi.encodeWithSelector(CallerNotBridgeMediator.selector));

        // Act
        manager.onTokenBurned(collection, 1);
    }

    function test_onTokenBurned_SuccessfulRelease() external {
        // Arrange
        collection.approve(address(nftVault), 1);
        vm.expectEmit(address(manager));
        emit MintTokenMessageSent(address(collection), 1, address(this));
        manager.deposit(collection, 1);
        assertEq(collection.ownerOf(1), address(nftVault));

        vm.expectEmit(address(manager));
        emit OnTokenBurnedCallback(address(collection), 1);

        // Act
        vm.prank(bridgeMediator);
        manager.onTokenBurned(collection, 1);

        // Assert
        assertEq(collection.ownerOf(1), address(this));
    }
}
