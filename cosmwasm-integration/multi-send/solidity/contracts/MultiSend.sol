//SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import { AxelarExecutableWithToken } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutableWithToken.sol';
import { IAxelarGateway } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol';
import { IERC20 } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol';
import { IAxelarGasService } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol';
import { SafeTokenTransfer, SafeTokenTransferFrom } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/libs/SafeTransfer.sol';
import {StringArray} from "./utils/stringArray.sol";

contract MultiSend is AxelarExecutableWithToken {
    using SafeTokenTransfer for IERC20;
    using SafeTokenTransferFrom for IERC20;

    IAxelarGasService public immutable gasReceiver;

    constructor(address gateway_, address gasReceiver_) AxelarExecutableWithToken(gateway_) {
        gasReceiver = IAxelarGasService(gasReceiver_);
    }

    function multiSend(
        string memory destinationChain,
        string memory destinationContract,
        string[] calldata recipients,
        string memory symbol,
        uint256 amount
    ) external payable {
        address tokenAddress = gatewayWithToken().tokenAddresses(symbol);
        IERC20(tokenAddress).safeTransferFrom(msg.sender, address(this), amount);
        IERC20(tokenAddress).approve(address(gatewayWithToken()), amount);

        bytes memory argValue = abi.encode(recipients);
        bytes memory payload  = abi.encode(
            "multi_send", // method name
            StringArray.fromArray1(["recipients"]), // argument name
            StringArray.fromArray1(["string[]"]), // argument type
            argValue // argument value
        );
        bytes memory payloadToCW = abi.encodePacked(
            bytes4(uint32(1)), // version number
            payload
        );

        // optional pay gas service
        if (msg.value > 0) {
            gasReceiver.payNativeGasForContractCallWithToken{value: msg.value}(
                address(this),
                destinationChain,
                destinationContract,
                payloadToCW,
                symbol,
                amount,
                msg.sender);
        }

        gatewayWithToken().callContractWithToken(destinationChain, destinationContract, payloadToCW, symbol, amount);
    }

    function _execute(bytes32 /*commandId*/, string calldata /*sourceChain*/, string calldata /*sourceAddress*/, bytes calldata /*payload*/) internal override {}

    function _executeWithToken(
        bytes32 /*commandId*/,
        string calldata /*sourceChain*/,
        string calldata /*sourceAddress*/,
        bytes calldata payload,
        string calldata tokenSymbol,
        uint256 amount
        ) internal override {
            // Demo only — this shouldn't be used as-is in production: it does not authenticate the
            // cross-chain message source. Validate sourceChain/sourceAddress against a trusted sender.
            address[] memory recipients = abi.decode(payload, (address[]));
            require(recipients.length > 0, "No recipients");
            address tokenAddress = gatewayWithToken().tokenAddresses(tokenSymbol);

            uint256 sentAmount = amount / recipients.length;
            for (uint256 i=0; i < recipients.length; i++) {
                IERC20(tokenAddress).safeTransfer(recipients[i], sentAmount);
            }
        }
}
