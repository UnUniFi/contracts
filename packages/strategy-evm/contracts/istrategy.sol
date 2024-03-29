// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IStrategy {
    function stake(uint256 amount) external;

    function unstake(uint256 amount) external;

    function epoch() external;
}
