Axelar provides two ways to send cross chain contract calls from an EVM chain to a cosmos chain.

#### - Encode the payload in the following format, and call Axelar gateway
```
bytes32 verison number
bytes   json serialized wasm contract calls
```

For example, if you want to call a wasm contract method HelloWorld
```
HelloWorld {
    greeting: String
}
```
You can use the js code snippet to send a cross chain contract call
```
var data = {
    hello_world: {
        greeting: "hello!"
    }
}

const payload = JSON.stringify(data);

const tx = await gateway.callContract(
    destChain, 
    destAddress,
    defaultAbiCoder.encode(
        [ "bytes32", "bytes" ],
        [hexZeroPad(hexlify(2), 32), toUtf8Bytes(payload)]
    )
);
await tx.wait();
```


#### - Encode the contract call on chain follows Axelar defined schema. 

As you may aware, CosmWasm uses JSON for contact calls which is different from evm. Axelar provides a method to translate abi encoded bytes to json serialized contract calls.

The smart contract needs to encode the payload in the following format in order to call a wasm contract
```
bytes32 verison number
bytes   actual payload, encoded in the following format:                
    string                   method name
    dynamic array of string  cosmwasm contract argument name
    dynamic array of string  argument abi types
    bytes                    abi encoded argument values
```

Reuse the hello world example, if you want to call a wasm contract method HelloWorld
```
HelloWorld {
    greeting: String
}
```
Enocde the payload in the following format and call Axelar gateway. Axelar will build wasm contract call from the abi encoded payload
```
bytes32 version = 0x0000000000000000000000000000000000000000000000000000000000000001;

bytes memory payload = abi.encode(
    version,
    abi.encode(
        "hello_world",
        ["greeting"],
        ["string"],
        abi.encode("hello!")
    )
);
```
