//SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IERC20CrossChain} from "./IERC20CrossChain.sol";
import {ERC20} from "@axelar-network/axelar-cgp-solidity/contracts/ERC20.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";
import {StringArray} from "./utils/stringArray.sol";
import {StringToAddress, AddressToString} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/utils/AddressString.sol";

contract ERC20CrossChain is AxelarExecutable, ERC20, IERC20CrossChain {
    using StringToAddress for string;
    using AddressToString for address;

    error AlreadyInitialized();
    error InvalidDestinationChain();

    event FalseSender(string sourceChain, string sourceAddress);

    IAxelarGasService public immutable gasReceiver;
    string public chainName;

    enum ChainType {
        EVM,
        COSMOS
    }
    mapping(string => ChainType) public supportedChains;

    constructor(
        address gateway_,
        address gasReceiver_,
        string memory name_,
        string memory symbol_,
        uint8 decimals_
    ) AxelarExecutable(gateway_) ERC20(name_, symbol_, decimals_) {
        gasReceiver = IAxelarGasService(gasReceiver_);
    }

    // This is for testing.
    function giveMe(uint256 amount) external {
        _mint(msg.sender, amount);
    }

    function transferRemote(
        string calldata destinationChain,
        string calldata destinationAddress,
        uint256 amount
    ) public payable override {
        _burn(msg.sender, amount);

        bytes memory payload;
        if (supportedChains[destinationChain] == ChainType.EVM) {
            payload = _encodePayloadToEVM(destinationAddress, amount);
        } else if (supportedChains[destinationChain] == ChainType.COSMOS) {
            payload = _encodePayloadToCW(destinationAddress, amount);
        } else {
            revert InvalidDestinationChain();
        }

        string memory stringAddress = address(this).toString();
        if (msg.value > 0) {
            gasReceiver.payNativeGasForContractCall{value: msg.value}(
                address(this),
                destinationChain,
                stringAddress,
                payload,
                msg.sender
            );
        }

        gateway.callContract(destinationChain, stringAddress, payload);
    }

    function _encodePayloadToEVM(
        string calldata destinationAddress,
        uint256 amount
    ) internal pure returns (bytes memory) {
        return abi.encode(destinationAddress, amount);
    }

    function _encodePayloadToCW(
        string calldata destinationAddress,
        uint256 amount
    ) internal view returns (bytes memory) {
        string memory sourceAddress = address(this).toString();
        bytes memory payload = _encodePayloadToEVM(destinationAddress, amount);
        bytes memory argValue = abi.encode(chainName, sourceAddress, payload);

        bytes memory payloadToCW = abi.encode(
            "execute",
            StringArray.fromArray3(
                ["source_chain", "source_address", "payload"]
            ),
            StringArray.fromArray3(["string", "string", "bytes"]),
            argValue
        );

        return
            abi.encode(
                bytes32(uint256(1)), // verison number
                payloadToCW
            );
    }

    function _execute(
        string calldata /*sourceChain*/,
        string calldata sourceAddress,
        bytes calldata payload
    ) internal override {
        if (sourceAddress.toAddress() != address(this)) {
            emit FalseSender(sourceAddress, sourceAddress);
            return;
        }
        (address to, uint256 amount) = abi.decode(payload, (address, uint256));
        _mint(to, amount);
    }

    function contractId() external pure returns (bytes32) {
        return keccak256("example");
    }
}
