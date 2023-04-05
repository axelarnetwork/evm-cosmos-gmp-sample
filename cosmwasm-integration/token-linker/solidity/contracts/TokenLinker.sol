//SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {ERC20} from "@axelar-network/axelar-cgp-solidity/contracts/ERC20.sol";
import {IERC20} from "./interfaces/IERC20.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";
import {StringToAddress, AddressToString} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/utils/AddressString.sol";
import {StringArray} from "./utils/stringArray.sol";

contract TokenLinker is AxelarExecutable {
    using StringToAddress for string;
    using AddressToString for address;

    error AlreadyInitialized();
    error InvalidDestinationChain();
    event FalseSender(string sourceChain, string sourceAddress);
    error GatewayToken();
    error AlreadyRegistered();
    error TransferFromFailed();
    error TransferFailed();

    IAxelarGasService public immutable gasService;
    string public chainName;
    address public tokenAddress;

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

    function transferToCosmos(
        string calldata destinationChain,
        string calldata destinationContract,
        string calldata recipient,
        uint256 amount
    ) public payable {
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

    function _execute(
        string calldata /*sourceChain*/,
        string calldata /*sourceAddress*/,
        bytes calldata payload
    ) internal override {
        // TODO: authenticaiton, anyone can call _execute atm
        (address to, uint256 amount) = abi.decode(payload, (address, uint256));
        _transfer(to, amount);
    }

    function _transfer(
        address to,
        uint256 amount
    ) internal {
        (bool success, bytes memory returnData) = tokenAddress.call(abi.encodeWithSelector(IERC20.transfer.selector, to, amount));
        bool transferred = success && (returnData.length == uint256(0) || abi.decode(returnData, (bool)));

        if (!transferred || tokenAddress.code.length == 0) revert TransferFailed();
    }
}
