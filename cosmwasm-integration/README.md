# CosmWasm Integration
This document provides instructions for integrating cross-ecosystem message passing between EVM and CosmWasm using the Axelar Network.

## Prerequisite
Ensure that the receiving chain has the general-purpose [IBC hook middleware](https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks) installed.

## EVM to CosmWasm

### Authenticate the Sender
Axelar Network uses a canonical account to facilitate GMP communication.
```
axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5
```

Upon packet arrival, the IBC hook middleware derives an intermediate account as the contract caller, based on the original sender and channel ID. This is a hash of `ibc-wasm-hook-intermediary (prefix key) + channel-id/axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5`

Since Axelar has a canonical IBC transfer channel for each chain, there is only one unique Axelar GMP sender on the receiving chain.

### Two Methods for Sending Cross-Chain Contract Calls
Axelar offers two methods for sending cross-chain contract calls from an EVM chain to CosmWasm:

#### 1. Pass JSON CosmWasm Contract Call Directly
The native method for calling contracts in the CosmWasm world is in JSON format. Axelar supports calling a CosmWasm contract using JSON-serialized contract call from an EVM chain. 

Developers can wrap the JSON contract call with a version number and send it to the Axelar gateway.

```
bytes32 verison number (0x0000000000000000000000000000000000000000000000000000000000000002)
bytes   json serialized wasm contract calls
```

For example, if you want to call a WASM contract method HelloWorld:
```
HelloWorld {
    greeting: String
}
```

You can use the following JavaScript code snippet to send a cross-chain contract call:
```javascript
var data = {
    hello_world: {
        greeting: "hello!"
    }
}

// '{"hello_world":{"greeting":"hello!"}"}'
const payload = JSON.stringify(data);

const tx = await gateway.callContract(
    destChain, 
    destAddress,
    defaultAbiCoder.encode(
        [ "bytes32", "bytes" ],
        [hexZeroPad(hexlify(2), 32), toUtf8Bytes(payload)] // wrap with version number
    )
);
await tx.wait();
```

#### 2.Encode the Contract Call On-Chain Using Axelar-Defined Schema
As you may already know, CosmWasm uses JSON serialization, which is different from Solidity. Axelar provides a method for translating ABI-encoded bytes to CosmWasm contract calls. 

Follow the encoding format provided below to ensure compatibility:
```
bytes32 verison number (0x0000000000000000000000000000000000000000000000000000000000000001)
bytes   ABI-encoded payload, indicating function name and arguments:           
            string                   CosmWasm contract method name
            dynamic array of string  CosmWasm contract argument name array
            dynamic array of string  argument abi type array
            bytes                    abi encoded argument values
```

Reusing the HelloWorld example:
```
HelloWorld {
    greeting: String
}
```

Enocde the payload in Solidity and call the Axelar gateway. Axelar will build the CosmWasm contract call from the ABI-encoded payload
```
bytes32 version = 0x0000000000000000000000000000000000000000000000000000000000000001;

string[] memory dynamicParamArray = new string[](1);
dynamicParamArray[0] = "greeting";

string[] memory dynamicTypeArray = new string[](1);
dynamicTypeArray[0] = "string";

bytes memory payload = abi.encode(
    version,
    abi.encode(
        "hello_world",
        dynamicParamArray,
        dynamicTypeArray,
        abi.encode("hello!")
    )
);
```

Here is mapping table between Solidity and CosmWasm types:
| CosmWasm       | Solidity | 
| :---:          | :---:    |
| bool           | bool     |
| u32            | uint     |
| i32            | int      |
| binary         | bytes    |
| vec            | array    |
| Uint64/128/256 | string   |

#### Encoding limitations
The current implementation only supports primitive types and does not complicated structs such as nested JSON.

Consider using JSON contract call to pass nested json, or use a generic CosmWasm contract function signature to handle general payloads.

```
Execute {
    source_chain: String,
    source_address: String,
    payload: Binary
}
```

### Attaching Source Chain and Address Info
In some cases, the contract call needs to know the source chain and address. Axelar reserves `source_chain` and `source_address` as keywords in wasm contract argument names. 

The CosmWasm contract can add `source_chain` and `source_address` arguments to the contract method when needed. 

Axelar validates the source chain and sender info if the payload contains these two arguments and rejects the cross-chain message if the source info is not the expected one.

The CosmWasm contract can trust the source info and use it to do some cross chain logic.