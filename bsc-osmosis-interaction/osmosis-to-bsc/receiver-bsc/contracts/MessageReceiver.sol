//SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import { AxelarExecutable } from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executables/AxelarExecutable.sol";

contract MessageReceiver is AxelarExecutable {
    // the lastest received message
    // every time a message is received, it will be updated
    string public lastReceivedMessage;

    // The gateway contract address should be taken from
    // https://docs.axelar.dev/resources/testnet#evm-contract-addresses
    // Ex Gateway on BSC: 0x4D147dCb984e6affEEC47e44293DA442580A3Ec0
    constructor(address gateway_) AxelarExecutable(gateway_) {}

    function _execute(
        string calldata sourceChain, 
        string calldata sourceAddress, 
        bytes calldata payload
    ) internal override {
        // decode the payload to get the message
        string memory message = abi.decode(payload, (string));

        // update the last received message by concatenating the new message with source address and chain
        lastReceivedMessage = string(abi.encodePacked("AT ", block.timestamp,
                                                      "RECEIVE MESSAGE: '", message, "'", 
                                                      " FROM ", sourceAddress, 
                                                      " ON ", sourceChain));
    }

    function _executeWithToken(
        string calldata sourceChain, 
        string calldata sourceAddress, 
        bytes calldata payload,
        string calldata tokenSymbol,
        uint256 amount
    ) internal override {
        // decode the payload to get the message
        string memory message = abi.decode(payload, (string));

        // update the last received message by concatenating the new message with source address and chain
        lastReceivedMessage = string(abi.encodePacked("AT ", block.timestamp,
                                                      "RECEIVE MESSAGE: '", message, "'", 
                                                      " FROM ", sourceAddress, 
                                                      " ON ", sourceChain,
                                                      " WITH ", amount, tokenSymbol));
    }
}
