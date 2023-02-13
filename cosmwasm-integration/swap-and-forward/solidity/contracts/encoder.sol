// SPDX-License-Identifier: MIT

pragma solidity 0.8.17;

struct Arg {
    string name;
    string ty;
    bytes value;
}

struct Payload {
    bytes32 ver;
    string func;
    bytes args;
}

contract Encoder {
    // theirs
    function arg_encode_ext() external pure returns (bytes memory) {
        Arg memory arg_1 = Arg({
            name: "a",
            ty: "uint256",
            value: abi.encode(6)
        });
        Arg memory arg_2 = Arg({
            name: "b",
            ty: "uint256",
            value: abi.encode(10)
        });
        Arg memory arg_3 = Arg({
            name: "c",
            ty: "string",
            value: abi.encode(
                "axelarvaloper1ntehswc7xmmxdukpt0mz3fuuj9r03g7p234pt5axelarvaloper1ntehswc7xmmxdukpt0mz3fuuj9r03g7p234pt5"
            )
        });

        Arg[] memory args = new Arg[](3);
        args[0] = arg_1;
        args[1] = arg_2;
        args[2] = arg_3;

        return encode("func1", args);
    }

    function str_encode_ext() external pure returns (bytes memory) {
        string[] memory arg_name = new string[](3);
        arg_name[0] = "a";
        arg_name[1] = "b";
        arg_name[2] = "c";

        string[] memory arg_type = new string[](3);
        arg_name[0] = "uint256";
        arg_name[1] = "uint256";
        arg_name[2] = "string";

        bytes memory arg_value = abi.encode(
            6,
            10,
            "axelarvaloper1ntehswc7xmmxdukpt0mz3fuuj9r03g7p234pt5axelarvaloper1ntehswc7xmmxdukpt0mz3fuuj9r03g7p234pt5"
        );

        return encodeStr("func1", arg_name, arg_type, arg_value);
    }

    // ours
    function encode(
        string memory funcName,
        Arg[] memory args
    ) internal pure returns (bytes memory) {
        return abi.encode(Payload({ver: bytes32(uint256(1)), func: funcName, args: abi.encode(args)}));
    }

    function encodeStr(
        string memory funcName,
        string[] memory arg_name,
        string[] memory arg_type,
        bytes memory arg_value
    ) internal pure returns (bytes memory) {
        return
            abi.encode(
                Payload({
                    ver: bytes32(uint256(1)),
                    func: funcName,
                    args: abi.encode(arg_name, arg_type, arg_value)
                })
            );
    }
}
