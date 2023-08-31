//SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {ERC20} from "@axelar-network/axelar-cgp-solidity/contracts/ERC20.sol";
import {IERC20} from "./interfaces/IERC20.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";
import {StringToAddress, AddressToString} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/utils/AddressString.sol";
import {StringArray} from "./utils/stringArray.sol";

contract MessageSender is AxelarExecutable {
    using StringToAddress for string;
    using AddressToString for address;

    error AlreadyInitialized();
    error InvalidDestinationChain();
    event FalseSender(string sourceChain, string sourceAddress);
    error GatewayToken();
    error AlreadyRegistered();
    error TransferFromFailed();
    error TransferFailed();

    // const AXELAR_GATEWAY = "0x4D147dCb984e6affEEC47e44293DA442580A3Ec0";
    // const CHAIN_NAME = "binance";
    // const AXELAR_GAS_RECEIVER = ethers.constants.AddressZero;
    // const TESTING_AURA_TOKEN = "0xDE41332a508E363079FD6993B81De049cD362B6D";
    IAxelarGasService public immutable gasService;
    string public chainName;
    address public tokenAddress;

    // The gateway contract address and gas receiver contract address should be taken from
    // https://docs.axelar.dev/resources/testnet#evm-contract-addresses
    // Ex Gateway on BSC: 0x4D147dCb984e6affEEC47e44293DA442580A3Ec0
    constructor(
        address gateway_, 
        address gasReceiver_, 
        string memory chainName_,
        address tokenAddress_
    ) AxelarExecutable(gateway_) {
        gasService = IAxelarGasService(gasReceiver_);
        chainName = chainName_;
        tokenAddress = tokenAddress_;
    }

    function SendMessage(
        string memory destinationChain, 
        string memory destinationContract, 
        string calldata recipient,
        uint256 amount
    ) external payable {
        _transferFrom(msg.sender, amount);

        bytes memory payload = _encodePayloadToCosmWasm(recipient, amount);
        _callContract(
            destinationChain,
            destinationContract,
            payload,
            msg.value
        );
    }

    function _transferFrom(address from, uint256 amount) internal {
        (bool success, bytes memory returnData) = tokenAddress.call(
            abi.encodeWithSelector(
                IERC20.transferFrom.selector,
                from,
                address(this),
                amount
            )
        );
        bool transferred = success &&
            (returnData.length == uint256(0) || abi.decode(returnData, (bool)));

        if (!transferred || tokenAddress.code.length == 0)
            revert TransferFromFailed();
    }

    function _encodePayloadToCosmWasm(
        string calldata destinationAddress,
        uint256 amount
    ) internal view returns (bytes memory) {
        bytes memory argValue = abi.encode(
            chainName,
            address(this).toString(),
            abi.encode(destinationAddress, amount)
        );

        bytes memory payload = abi.encode(
            "execute_from_remote",
            StringArray.fromArray3(
                ["source_chain", "source_address", "payload"]
            ),
            StringArray.fromArray3(["string", "string", "bytes"]),
            argValue
        );

        return
            abi.encodePacked(
                bytes4(0x00000001), // verison number
                payload
            );
    }

    function _callContract(
        string memory destinationChain,
        string memory destinationAddress,
        bytes memory payload,
        uint256 gasValue
    ) internal {
        if (gasValue > 0) {
            gasService.payNativeGasForContractCall{value: gasValue}(
                address(this),
                destinationChain,
                destinationAddress,
                payload,
                msg.sender
            );
        }
        gateway.callContract(destinationChain, destinationAddress, payload);
    }

    // function SendMessageWithToken(
    //     string memory destinationChain, 
    //     string memory destinationContract, 
    //     string memory message,
    //     string memory symbol,
    //     uint256 amount
    // ) external payable {
    //     // Send token to this contract then let Gateway to transfer it to the destination chain
    //     address tokenAddress = gateway.tokenAddresses(symbol);
    //     IERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);
    //     IERC20(tokenAddress).approve(address(gateway), amount);

    //     // Define the payload to cosmwasm
    //     bytes memory payload  = abi.encode(
    //         "receive_message_with_token", // method name
    //         "message", // argument name
    //         "string", // argument type
    //         message // argument value
    //     );

    //     // Encode the payload to cosmwasm
    //     bytes memory payloadToCW = abi.encode(
    //         bytes32(uint256(1)), // verison number
    //         payload
    //     );

    //     // Is this neccessary?
    //     // In yes, how many gas should be sent? Is that the reason why this fuction is payable?
    //     // In no, who pays the gas in the destination chain?
    //     if (msg.value > 0) {
    //         gasReceiver.payNativeGasForContractCallWithToken{value: msg.value}(
    //             address(this), 
    //             destinationChain, 
    //             destinationContract, 
    //             payloadToCW, 
    //             symbol, 
    //             amount, 
    //             msg.sender);
    //     }

    //     gateway.callContractWithToken(destinationChain, destinationContract, payloadToCW, symbol, amount);
    // }
}
