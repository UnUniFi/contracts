// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";

contract YieldAggregatorOutpost is AxelarExecutable {
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

    function depositToVault(
        string calldata destinationChain,
        string calldata destinationAddress,
        string calldata depositor,
        uint64 vaultId,
        address erc20,
        uint256 amount
    ) external payable {
        require(msg.value > 0, "Gas payment is required");

        IERC20 tokenContract = IERC20(erc20);
        tokenContract.transferFrom(msg.sender, address(this), amount);

        bytes memory payload = _encodePayloadToCosmWasm(depositor, vaultId);
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
        string calldata depositor,
        uint64 vaultId
    ) internal view returns (bytes memory) {
        // Schema
        //   bytes4  version number (0x00000001)
        //   bytes   ABI-encoded payload, indicating function name and arguments:
        //     string                   CosmWasm contract method name
        //     dynamic array of string  CosmWasm contract argument name array
        //     dynamic array of string  argument abi type array
        //     bytes                    abi encoded argument values

        // contract call arguments for ExecuteMsg::receive_message_evm{ source_chain, source_address, payload }
        bytes memory argValues = abi.encode(depositor, vaultId);

        string[] memory argumentNameArray = new string[](2);
        argumentNameArray[0] = "depositor";
        argumentNameArray[1] = "vault_id";

        string[] memory abiTypeArray = new string[](2);
        abiTypeArray[0] = "string";
        abiTypeArray[1] = "uint64";

        bytes memory gmpPayload;
        gmpPayload = abi.encode(
            "deposit_to_vault",
            argumentNameArray,
            abiTypeArray,
            argValues
        );

        return abi.encodePacked(bytes4(0x00000001), gmpPayload);
    }

    function _executeWithToken(
        string calldata,
        string calldata,
        bytes calldata payload,
        string calldata tokenSymbol,
        uint256 amount
    ) internal override {
        // TODO: verify sourceChain_ and sourceAddress_
        // sourceAddress_ must be the outpost internal contract.
        address recipient = abi.decode(payload, (address));
        address tokenAddress = gateway.tokenAddresses(tokenSymbol);

        IERC20(tokenAddress).transfer(recipient, amount);
    }
}
