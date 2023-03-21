//SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import { AxelarExecutable } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/executables/AxelarExecutable.sol';
import { IAxelarGateway } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol';
import { IERC20 } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol';
import { IAxelarGasService } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol';


contract MultiSend is AxelarExecutable {
    IAxelarGasService public immutable gasReceiver;

    constructor(address gateway_, address gasReceiver_) AxelarExecutable(gateway_) {
        gasReceiver = IAxelarGasService(gasReceiver_);
    }

    function multiSend(
        string memory destinationChain,
        string memory destinationAddress,
        string[] calldata receiverAddresses,
        string memory symbol,
        uint256 amount
    ) external payable {
        address tokenAddress = gateway.tokenAddresses(symbol);
        IERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);
        IERC20(tokenAddress).approve(address(gateway), amount);
        
        bytes memory payloadWithVersion = abi.encode(
            bytes32(uint256(0)), // version number
            abi.encode(receiverAddresses)
        );

        // optional pay gas service
        if (msg.value > 0) {
            gasReceiver.payNativeGasForContractCallWithToken{value: msg.value}(
                address(this), 
                destinationChain, 
                destinationAddress, 
                payloadWithVersion, 
                symbol, 
                amount, 
                msg.sender);
        }

        gateway.callContractWithToken(destinationChain, destinationAddress, payloadWithVersion, symbol, amount);
    }

    function _executeWithToken(
        string calldata sourceChain, 
        string calldata sourceAddress, 
        bytes calldata payload, 
        string calldata tokenSymbol,
        uint256 amount
        ) internal override {
            address[] memory recipients = abi.decode(payload, (address[]));
            address tokenAddress = gateway.tokenAddresses(tokenSymbol);

            uint256 sentAmount = amount / recipients.length;
            for (uint256 i=0; i < recipients.length; i++) {
                IERC20(tokenAddress).transfer(recipients[i], sentAmount);
            }
        }
}
