## EVM <-> Cosmos General Message Passing Demo

Axelar enables general messages passing between EVM and Cosmos chains.

In a nutshell, Axelar verifies messages originating from an EVM chain and forwards them as memo fields through ICS20 packets.

Two integration methods are currently supported:
1. **Native Chain Integration**: Axelar verifies and forwards arbitrary payloads to the destination Cosmos chain. The receiving chain must implement an IBC middleware with a custom handler to decode and process the payload. For more information, please refer to the [Native integration guide](./native-integration/README.md).

- **CosmWasm Integration**: For chains with enabled Wasm modules, Axelar supports calling a CosmWasm contract from an EVM smart contract ([with some limitations](./cosmwasm-integration/README.md#encoding-limitations)). The receiving chain must install the general-purpose [IBC hook middleware](https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks). Message sender have the option to either encode the payload using Axelar's defined schema or pass the JSON CosmWasm contract call directly. Axelar verifies the payload and translates it into a Wasm execute message. For more details, please refer to the [CosmWasm integration guide](./cosmwasm-integration/README.md).
