// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "openzeppelin-contracts/contracts/token/ERC721/IERC721.sol";
import "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {AxelarExecutable} from "axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IAxelarGasService} from "axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";

contract NftBackedLoan {
    IERC20 public immutable usdc;
    IAxelarGasService public immutable gasService;

    mapping(address => mapping(uint256 => address)) public nftListers;
    uint256 public constant NO_MIN = type(uint256).max;

    constructor(
        address usdc_,
        address gateway_,
        address gasReceiver_
    ) AxelarExecutable(gateway_) {
        usdc = IERC20(usdc_);
        gasService = IAxelarGasService(gasReceiver_);
    }

    function listNft(
        string calldata destinationChain,
        string calldata destinationAddress,
        address operator,
        uint256 tokenId,
        uint256 minDeposit
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        // Collateralize NFT
        IERC721(operator).transferFrom(msg.sender, address(this), tokenId);
        nftListers[operator][tokenId] = msg.sender;

        bytes memory payload = abi.encode(value_);
        gasService.payNativeGasForContractCall{value: msg.value}(
            address(this),
            destinationChain,
            destinationAddress,
            payload,
            msg.sender
        );
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    function borrow(
        string calldata destinationChain,
        string calldata destinationAddress,
        uint256 amount
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        // Just messaging to the gateway that the sender want to borrow USDC
        // Destination CosmWasm will send back USDC to this contract in _executeWithToken

        bytes memory payload = abi.encode(value_);
        gasService.payNativeGasForContractCall{value: msg.value}(
            address(this),
            destinationChain,
            destinationAddress,
            payload,
            msg.sender
        );
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    function repay(
        string calldata destinationChain,
        string calldata destinationAddress,
        uint256 amount
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        // Repay USDC to this contract
        usdc.transferFrom(msg.sender, address(this), amount);
        // This contract approves the gateway to use USDC
        usdc.approve(address(gateway), amount);

        bytes memory payload = abi.encode(value_);
        gasService.payNativeGasForContractCall{value: msg.value}(
            address(this),
            destinationChain,
            destinationAddress,
            payload,
            msg.sender
        );
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    function _sendBorrowedAmount(
        address recipient,
        uint256 amount
    ) internal virtual {
        usdc.transfer(recipient, amount);
    }

    function _endNftListing(
        address operator,
        uint256 tokenId
    ) internal virtual {
        address auctioneer = nftListers[operator][tokenId];
        require(auctioneer != address(0), "NOT_AUCTIONING");
        minAmounts[operator][tokenId] = 0;
        nftListers[operator][tokenId] = address(0);
    }

    function _withdrawNft(
        address recipient,
        address operator,
        uint256 tokenId
    ) internal virtual {
        IERC721(operator).transferFrom(address(this), recipient, tokenId);
    }

    function _execute(
        string calldata sourceChain_,
        string calldata sourceAddress_,
        bytes calldata payload_
    ) internal override {
        (value) = abi.decode(payload_, (string));
        sourceChain = sourceChain_;
        sourceAddress = sourceAddress_;

        // switch case
        // case endNftListing
        //   call _endNftListing
        // case withdrawNft
        //   call _withdrawNft
        // default
        //   error
    }

    // Tokens are coming from the gateway to this contract
    function _executeWithToken(
        string calldata,
        string calldata,
        bytes calldata payload,
        string calldata tokenSymbol,
        uint256 amount
    ) internal override {
        // switch
        // case sendBorrowedAmount
        //   call _sendBorrowedAmount
        // default
        //   error
    }
}
