## EVM <-> Cosmos Generam Message Passing Demo

Axelar supports pass general message from EVM chains to Cosmos chains. Axelar confirms the message from a EVM chain and forward the message as memo field via ICS20 packet.
It currently support two integration methods:
- Native chain: Axelar confirms and forward arbitrary payload to the destiantion cosmos chian. The receiver chain needs to add a middleware and implement a customize handler to decode and process the payload.
- Cosmwasm: The message sender from evm chain needs to encode the payload in Axelar defined schema. Axelar confirms the payload and transates to wasm execute message. The receiver chain needs to add a general purpose ibc hook middle, which calls wasm to execute the message.

This repo contains examples for both native and wasm integration.
- multi send is a native integration example, it sends token to multiple recipients from evm chain to a cosmos chain.
- swap and forward sends token and calls a cosmwasm swap contract on Osmosis

### EVM
Developer needs to deploy a smart contract that calls Axelar gateway to send cross-chain message,
and decode message upon receiving the payload from a remote chain.

[MultiSend.sol](./multi-send-solidity/contracts/MultiSend.sol) is a sample contract.

[deploy.js](./multi-send-solidity/deploy.js) and [interact.js](./multi-send-solidity/interact.js) are scripts to deploy and interact with the sample contract.

### Cosmos Native Integration
A Cosmos chain can integrate cross chain message passing by adding an [IBC middleware](./multi-send-cosmos-native/gmpdemo/ibc_middleware.go),
and a handler that implements [GeneralMessageHandler](./multi-send-cosmos-native/gmpdemo/gmp_handler.go#L33) interface to decode the paylod. [MultiSendHandler](./multi-send-cosmos-native/gmpdemo/keeper/multi_send_handler.go#L27) is a sample implementation.

[MultiSend](./multi-send-cosmos-native/gmpdemo/keeper/msg_server.go) is a sample transaction that sends payload to an EVM chain.

### Wasm Integration
The smart contract needs to encode the payload in the following format in order to call a wasm contract
```
 bytes memory argValue = abi.encode(arg1, arg2, arg3...)

 bytes memory payload = abi.encde(
    wasm contract method name,
    argument name list,
    argument type list,
    argValue bytes
)
```
Axelar also reserves `source_chain` and `source_address` keywords in wasm contract argument name. Axelar replaces with its canonical chain and sender info if the contract call contains these arguments. 