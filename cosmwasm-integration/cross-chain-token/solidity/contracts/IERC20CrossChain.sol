// SPDX-License-Identifier: MIT

pragma solidity 0.8.9;

import { IERC20 } from '@axelar-network/axelar-cgp-solidity/contracts/interfaces/IERC20.sol';

interface IERC20CrossChain is IERC20 {
    function transferRemote(
        string calldata destinationChain,
        string calldata destinationAddress,
        uint256 amount
    ) external payable;
}