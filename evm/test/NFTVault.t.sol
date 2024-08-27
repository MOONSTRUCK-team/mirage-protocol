// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25 <0.9.0;

import { Test } from "forge-std/src/Test.sol";

import { NFTVault } from "../src/NFTVault.sol";
import { NFTExample } from "./NFTExample.sol";

import { TokenAlreadyDeposited, TokenNotDeposited } from "../src/NFTVaultErrors.sol";
import { TokenDeposited, TokenReleased } from "../src/NFTVaultEvents.sol";

contract NFTVaultTest is Test {
    NFTVault internal nftVault;
    NFTExample internal collection;

    function setUp() public virtual {
        nftVault = new NFTVault();
        collection = new NFTExample();
    }

    function test_deposit_RevertIf_TokenAlreadyDeposited() external {
        // Arrange
        collection.approve(address(nftVault), 1);
        vm.expectEmit(address(nftVault));
        emit TokenDeposited(address(collection), 1, address(this));
        nftVault.deposit(collection, 1);
        assertEq(collection.ownerOf(1), address(nftVault));

        // Assert
        vm.expectRevert(abi.encodeWithSelector(TokenAlreadyDeposited.selector, address(collection), 1));

        // Act
        nftVault.deposit(collection, 1);
    }

    function test_deposit_SuccessfulDeposit() external {
        // Arrange
        collection.approve(address(nftVault), 1);

        vm.expectEmit(address(nftVault));
        emit TokenDeposited(address(collection), 1, address(this));

        // Act
        nftVault.deposit(collection, 1);

        // Assert
        assertEq(collection.ownerOf(1), address(nftVault));
    }

    function test_release_RevertIf_TokenNotDeposited() external {
        // Arrange

        // Assert
        vm.expectRevert(abi.encodeWithSelector(TokenNotDeposited.selector, address(collection), 1));

        // Act
        nftVault.release(collection, 1);
    }

    function test_release_SuccessfulRelease() external {
        // Arrange
        collection.approve(address(nftVault), 1);
        vm.expectEmit(address(nftVault));
        emit TokenDeposited(address(collection), 1, address(this));
        nftVault.deposit(collection, 1);
        assertEq(collection.ownerOf(1), address(nftVault));

        vm.expectEmit(address(nftVault));
        emit TokenReleased(address(collection), 1, address(this));

        // Act
        nftVault.release(collection, 1);

        // Assert
        assertEq(collection.ownerOf(1), address(this));
    }
}
