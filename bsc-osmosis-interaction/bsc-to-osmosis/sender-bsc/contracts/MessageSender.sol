//SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import { AxelarExecutable } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/executables/AxelarExecutable.sol';
import { IAxelarGateway } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol';
import { IERC20 } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol';
import { IAxelarGasService } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol';

contract MessageSender is AxelarExecutable {
    IAxelarGasService public immutable gasReceiver;

    // The gateway contract address and gas receiver contract address should be taken from
    // https://docs.axelar.dev/resources/testnet#evm-contract-addresses
    // Ex Gateway on BSC: 0x4D147dCb984e6affEEC47e44293DA442580A3Ec0
    constructor(address gateway_, address gasReceiver_) AxelarExecutable(gateway_) {
        gasReceiver = IAxelarGasService(gasReceiver_);
    }

    function SendMessage(
        string memory destinationChain, 
        string memory destinationContract, 
        string memory message
    ) external {
        // Define the payload to cosmwasm,
        bytes memory payload  = abi.encode(
            "send_message", // method name
            "message", // argument name
            "string", // argument type
            message // argument value
        );

        // Encode the payload to cosmwasm
        bytes memory payloadToCW = abi.encode(
            bytes32(uint256(1)), // verison number
            payload
        );

        // Is this neccessary?
        // In yes, how many gas should be sent? Is that the reason why this fuction is payable?
        // In no, who pays the gas in the destination chain?
        gasReceiver.payNativeGasForContractCall(
            address(this),
            destinationChain,
            destinationContract,
            payloadToCW,
            msg.sender
        );

        gateway.callContract(destinationChain, destinationContract, payloadToCW);
    }

    function SendMessageWithToken(
        string memory destinationChain, 
        string memory destinationContract, 
        string memory message,
        string memory symbol,
        uint256 amount
    ) external payable {
        // Send token to this contract then let Gateway to transfer it to the destination chain
        address tokenAddress = gateway.tokenAddresses(symbol);
        IERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);
        IERC20(tokenAddress).approve(address(gateway), amount);

        // Define the payload to cosmwasm
        bytes memory payload  = abi.encode(
            "send_message_with_token", // method name
            "message", // argument name
            "string", // argument type
            message // argument value
        );

        // Encode the payload to cosmwasm
        bytes memory payloadToCW = abi.encode(
            bytes32(uint256(1)), // verison number
            payload
        );

        // Is this neccessary?
        // In yes, how many gas should be sent? Is that the reason why this fuction is payable?
        // In no, who pays the gas in the destination chain?
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

        gateway.callContractWithToken(destinationChain, destinationContract, payloadToCW, symbol, amount);
    }
}
