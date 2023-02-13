// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

library StringArray {
    function fromArray5(
        string[5] memory array
    ) internal pure returns (string[] memory) {
        require(array.length == 5, "array length != 5");

        string[] memory array2 = new string[](5);
        for (uint i = 0; i < array.length; i++) {
            array2[i] = array[i];
        }
        return array2;
    }
}
