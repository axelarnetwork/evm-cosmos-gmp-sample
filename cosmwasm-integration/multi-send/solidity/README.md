## Introduction
The contract calls a cosmwasm contract deployed on a cosmos chain. It calls the [MultiSend](../cosmwasm/src/msg.rs#12) method, which takes a list of string address.

It first encodes the argument value in byes
```
bytes memory argValue = abi.encode(recipients);
```

Then it constructs the payload following Axelar defined schema, so that Axelar can decode the payload and construct the cosmwasm contract call.
```
bytes memory payload  = abi.encode(
    "multi_send",                           // method name,    used to construct wasm contract call
    StringArray.fromArray1(["recipients"]), // argument name,  used to construct wasm contract call
    StringArray.fromArray1(["string[]"]),   // argument type,  used to unpack the argume values
    argValue                                // argument value, used to construct wasm contract call
);
``` 

After that, it wraps the payload with verison number and calls Axelar gateway.
```
bytes memory payloadToCW = abi.encode(
    bytes32(uint256(1)), // verison number
    payload
);

gateway.callContractWithToken(destinationChain, destinationContract, payloadToCW, symbol, amount);
```

## Setup
```bash
npm i
npm run build
```

## Deplpy
```bash
node deploy.js
```

