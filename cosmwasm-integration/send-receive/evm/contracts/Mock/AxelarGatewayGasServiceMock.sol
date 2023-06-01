// SPDX-License-Identifier: MIT

//! AxelarGatewayGasServiceMock mocks out the Axelar Gateway and Gas Service contracts for local testing.
//! This contract contains no business logic, and just drops calls to {payNativeGas()} and {callContract()}
//! A custom function, {executeFromGateway()}, is added so the test can call {_execute()} on the main contract.

pragma solidity ^0.8.0;
// import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";
// import { IAxelarGateway } from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IAxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarExecutable.sol";
import {StringToAddress, AddressToString} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/utils/AddressString.sol";

interface IMessengerGetter {
    function messenger() external returns (address);
}

contract AxelarGatewayGasServiceMock {
    using StringToAddress for string;
    using AddressToString for address;

    bytes public storedPayload; // stores payload to callContract for the test to manually validate.

    function callContract(
        string calldata /*destinationChain*/,
        string calldata /*contractAddress*/,
        bytes calldata payload
    ) external {
        storedPayload = payload;
        // IAxelarExecutable(
        //     IMessengerGetter(contractAddress.toAddress()).messenger()
        // ).execute("0x", destinationChain, contractAddress, payload);
        // no op
    }

    function payNativeGasForContractCall(
        address /*sender*/,
        string calldata /*destinationChain*/,
        string calldata /*destinationAddress*/,
        bytes calldata /*payload*/,
        address /*refundAddress*/
    ) external payable {}

    function validateContractCall(
        bytes32 /*commandId*/,
        string calldata /*sourceChain*/,
        string calldata /*sourceAddress*/,
        bytes32 /*payloadHash*/
    ) external pure returns (bool) {
        return true;
    }

    function executeFromGateway(
        address destinationAddress,
        string calldata sourceChain,
        string calldata sourceAddress,
        bytes calldata payload
    ) external {
        IAxelarExecutable(destinationAddress).execute("0x", sourceChain, sourceAddress, payload);
    }
}
