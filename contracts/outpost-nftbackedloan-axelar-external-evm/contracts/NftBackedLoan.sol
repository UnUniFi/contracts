// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";

contract NftBackedLoan is AxelarExecutable {
    IAxelarGasService public immutable gasService;
    string public chainName; // name of the chain this contract is deployed to

    constructor(
        address gateway_,
        address gasReceiver_,
        string memory chainName_
    ) AxelarExecutable(gateway_) {
        gasService = IAxelarGasService(gasReceiver_);
        chainName = chainName_;
    }

    function listNft(
        string calldata destinationChain,
        string calldata destinationAddress,
        address nftContract,
        uint256 tokenId,
        string calldata ununifiAddress,
        string calldata bidDenom,
        uint128 minDepositRateDecimal6,
        uint128 minBidPeriodSeconds
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        // Collateralize NFT
        IERC721(nftContract).transferFrom(msg.sender, address(this), tokenId);

        bytes memory executeMsgPayload = abi.encode(
            "list_nft",
            nftContract,
            tokenId,
            ununifiAddress,
            bidDenom,
            minDepositRateDecimal6,
            minBidPeriodSeconds
        );
        bytes memory payload = _encodePayloadToCosmWasm(executeMsgPayload);
        gasService.payNativeGasForContractCall{value: msg.value}(
            address(this),
            destinationChain,
            destinationAddress,
            payload,
            msg.sender
        );
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    function _encodePayloadToCosmWasm(
        address nftContract,
        uint256 tokenId,
        string calldata ununifiAddress,
        string calldata bidDenom,
        uint128 minDepositRateDecimal6,
        uint128 minBidPeriodSeconds
    ) internal view returns (bytes memory) {
        // Schema
        //   bytes4  version number (0x00000001)
        //   bytes   ABI-encoded payload, indicating function name and arguments:
        //     string                   CosmWasm contract method name
        //     dynamic array of string  CosmWasm contract argument name array
        //     dynamic array of string  argument abi type array
        //     bytes                    abi encoded argument values

        // contract call arguments for ExecuteMsg::receive_message_evm{ source_chain, source_address, payload }
        bytes memory argValues = abi.encode(
            chainName,
            address(this),
            nftContract.toString(), // TODO
            tokenId.toString(),
            ununifiAddress,
            bidDenom,
            minDepositRateDecimal6.toString(),
            minBidPeriodSeconds.toString()
        );

        string[] memory argumentNameArray = new string[](3);
        argumentNameArray[0] = "source_chain";
        argumentNameArray[1] = "source_address";
        argumentNameArray[2] = "class_id";
        argumentNameArray[3] = "token_id";
        argumentNameArray[4] = "ununifi_address";
        argumentNameArray[5] = "bid_denom";
        argumentNameArray[6] = "min_deposit_rate_decimal6";
        argumentNameArray[7] = "min_bid_period_seconds";

        string[] memory abiTypeArray = new string[](3);
        abiTypeArray[0] = "string";
        abiTypeArray[1] = "string";
        abiTypeArray[2] = "string";
        abiTypeArray[3] = "string";
        abiTypeArray[4] = "string";
        abiTypeArray[5] = "string";
        abiTypeArray[6] = "string";
        abiTypeArray[7] = "string";

        bytes memory gmpPayload;
        gmpPayload = abi.encode(
            "list_nft",
            argumentNameArray,
            abiTypeArray,
            argValues
        );

        return abi.encodePacked(bytes4(0x00000001), gmpPayload);
    }

    function _sendBackNft(
        address recipient,
        address nftContract,
        uint256 tokenId
    ) internal virtual {
        IERC721(nftContract).transferFrom(address(this), recipient, tokenId);
    }

    function _execute(
        string calldata sourceChain_,
        string calldata sourceAddress_,
        bytes calldata payload_
    ) internal override {
        // TODO: verify sourceChain_ and sourceAddress_
        (
            string memory destinationAddress,
            string memory classId,
            string memory tokenId
        ) = abi.decode(payload_, (string, string, string));

        address recipient = address(destinationAddress);
        address nftContract = address(classId);
        uint256 tokenIdInt = uint256(tokenId);

        _sendBackNft(recipient, nftContract, tokenIdInt);
    }
}
