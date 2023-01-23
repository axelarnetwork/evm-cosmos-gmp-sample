//SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import {AxelarExecutable} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executables/AxelarExecutable.sol";
import {IAxelarGateway} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol";
import {IERC20} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol";
import {IAxelarGasService} from "@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract SwapAndForward is AxelarExecutable {
    IAxelarGasService public immutable gasReceiver;
    string public swapContract;
    uint constant swapContractArgNum = 5;

    constructor(
        address gateway_,
        address gasReceiver_,
        string memory swapContract_
    ) AxelarExecutable(gateway_) {
        gasReceiver = IAxelarGasService(gasReceiver_);
        swapContract = swapContract_;
    }

    function swapAndForward(
        string memory receiverChain,
        string memory receiverAddress,
        string memory symbol,
        uint256 amount,
        string memory outputCoin,
        uint8 slippage
    ) external payable {
        address tokenAddress = gateway.tokenAddresses(symbol);
        IERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);
        IERC20(tokenAddress).approve(address(gateway), amount);

        bytes memory payload = encode(
            receiverChain,
            receiverAddress,
            amount,
            outputCoin,
            slippage
        );

        // optional pay gas service
        if (msg.value > 0) {
            gasReceiver.payNativeGasForContractCallWithToken{value: msg.value}(
                address(this),
                "osmosis",
                swapContract,
                payload,
                symbol,
                amount,
                msg.sender
            );
        }

        gateway.callContractWithToken(
            "osmosis",
            swapContract,
            payload,
            symbol,
            amount
        );
    }

    function encode(
        string memory receiverChain,
        string memory receiverAddress,
        uint256 amount,
        string memory outputCoin,
        uint8 slippage
    ) internal pure returns (bytes memory) {
        string[] memory argName = new string[](5);
        argName[0] = "dest_chain";
        argName[1] = "dest_address";
        argName[2] = "swap_amount";
        argName[3] = "output_denom";
        argName[4] = "slippage";

        string[] memory argType = new string[](5);
        argType[0] = "string";
        argType[1] = "string";
        argType[2] = "string";
        argType[3] = "string";
        argType[4] = "uint8";

        bytes memory args = abi.encode(
            receiverChain,
            receiverAddress,
            Strings.toString(amount),
            outputCoin,
            slippage
        );

        bytes memory payload = abi.encode(
            argName, // wasm contract method arg names
            argType, // argument types
            args // argument bytes
        );

        return encode_("swap_and_forward", payload);
    }

    function encode_(
        string memory funcName,
        bytes memory payload
    ) internal pure returns (bytes memory) {
        return
            abi.encode(
                bytes32(uint256(1)), // inidicates payload to wasm
                funcName,
                payload
            );
    }

    function _executeWithToken(
        string calldata sourceChain,
        string calldata sourceAddress,
        bytes calldata payload,
        string calldata tokenSymbol,
        uint256 amount
    ) internal override {}
}
