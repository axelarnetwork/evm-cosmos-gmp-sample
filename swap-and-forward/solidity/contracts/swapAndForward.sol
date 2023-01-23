//SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import { AxelarExecutable } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/executables/AxelarExecutable.sol';
import { IAxelarGateway } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol';
import { IERC20 } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol';
import { IAxelarGasService } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol';
import '@openzeppelin/contracts/utils/Strings.sol';


contract SwapAndForward is AxelarExecutable {
    IAxelarGasService public immutable gasReceiver;
    string public swapContract;
    uint constant swapContractArgNum = 5;

    constructor(address gateway_, address gasReceiver_, string memory swapContract_) AxelarExecutable(gateway_) {
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
        
       
        string[swapContractArgNum] memory argNames = ['dest_chain', 'dest_address', 'swap_amount', 'output_denom', 'slippage'];
        string[swapContractArgNum] memory argTypes = ['string', 'string', 'string', 'string', 'uint8'];
        
        bytes memory args =  abi.encode(
            receiverChain,
            receiverAddress,
            Strings.toString(amount),
            outputCoin,
            slippage
        );

        bytes memory payload =  abi.encode(
            'swap_and_forward', // wasmd contract method name,
            argNames, // wasm contract method arg names
            argTypes, // argument types
            args // argument bytes
        );

        bytes memory payload2 = abi.encode(
            bytes32(uint256(1)), // inidicates payload to wasm
            swapContractArgNum,
            payload
        );
        
        // optional pay gas service
        if (msg.value > 0) {
            gasReceiver.payNativeGasForContractCallWithToken{value: msg.value}(
                address(this), 
                "osmosis", 
                swapContract, 
                payload2, 
                symbol, 
                amount, 
                msg.sender
            );
        }

        gateway.callContractWithToken("osmosis", swapContract, payload2, symbol, amount);
    }

    function _executeWithToken(
        string calldata sourceChain, 
        string calldata sourceAddress, 
        bytes calldata payload, 
        string calldata tokenSymbol,
        uint256 amount
        ) internal override {
        }
}
