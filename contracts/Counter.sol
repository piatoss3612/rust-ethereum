// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

contract Counter {
    uint256 public count;

    function increment() public {
        count += 1;
    }
}
