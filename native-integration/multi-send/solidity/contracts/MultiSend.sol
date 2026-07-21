//SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import { AxelarExecutableWithToken } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutableWithToken.sol';
import { IAxelarGateway } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol';
import { IERC20 } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol';
import { IAxelarGasService } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGasService.sol';
import { SafeTokenTransfer, SafeTokenTransferFrom } from '@axelar-network/axelar-gmp-sdk-solidity/contracts/libs/SafeTransfer.sol';


contract MultiSend is AxelarExecutableWithToken {
    using SafeTokenTransfer for IERC20;
    using SafeTokenTransferFrom for IERC20;

    IAxelarGasService public immutable gasReceiver;

    constructor(address gateway_, address gasReceiver_) AxelarExecutableWithToken(gateway_) {
        gasReceiver = IAxelarGasService(gasReceiver_);
    }

    function multiSend(
        string memory destinationChain,
        string memory destinationAddress,
        string[] calldata receiverAddresses,
        string memory symbol,
        uint256 amount
    ) external payable {
        address tokenAddress = gatewayWithToken().tokenAddresses(symbol);
        IERC20(tokenAddress).safeTransferFrom(msg.sender, address(this), amount);
        IERC20(tokenAddress).approve(address(gatewayWithToken()), amount);

        bytes memory payloadWithVersion = abi.encodePacked(
            bytes4(uint32(0)), // version number
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

        gatewayWithToken().callContractWithToken(destinationChain, destinationAddress, payloadWithVersion, symbol, amount);
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
            // This handler is intentionally permissionless, and that is safe here: it only distributes
            // the tokens delivered with THIS message (`amount`) among the payload-supplied recipients
            // and holds no funds or privileged state, so a forged call can only move the caller's own
            // delivered tokens (sentAmount * len <= amount). Authenticating the source would buy
            // nothing. Add source authentication when a forged message could command value or state it
            // is not entitled to — see token-linker in this repo, which mints and must verify the sender.
            address[] memory recipients = abi.decode(payload, (address[]));
            require(recipients.length > 0, "No recipients");
            address tokenAddress = gatewayWithToken().tokenAddresses(tokenSymbol);

            uint256 sentAmount = amount / recipients.length;
            for (uint256 i=0; i < recipients.length; i++) {
                IERC20(tokenAddress).safeTransfer(recipients[i], sentAmount);
            }
        }
}
