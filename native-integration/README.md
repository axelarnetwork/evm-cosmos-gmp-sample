
## GMP between EVM and Cosmos chains.

Axelar uses a canonical account `axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5` for gmp communication.
The recipient chain can use channel-id and sender address to authenticate the message.

### EVM -> Cosmos
wrap the payload with version number, and call Axelar gateway contract.
```
bytes32 verison number (0 for native)
bytes   payload
```
e.g.
```
bytes32 version = 0x0000000000000000000000000000000000000000000000000000000000000000;

bytes memory payload = abi.encode(
    version,
    abi.encode(...)
);

gateway.callContractWithToken(destinationChain, destinationAddress, payload, symbol, amount);
```

Axelar confirms the message, attaches source chain and address info, and sends the message to the destination chain via ics20 memo.

Message is a json struct, contains source_chain, source_address and payload, e.g.
```json
{
  "source_chain": "Ethereum",
  "source_address": "0x777d2D82dAb1BF06a4dcf5a3E07057C41100c22D",
  "payload": ...,
  "type": 1
}
```
type field indicates message type
- `1` pure message
- `2` message with token

The recipient chain has freedom to process the payload upon packet arrival.

### Cosmos -> EVM
Send a ibc transfer to `axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5`, attach the message in the memo field.
The message indicates the destination chain and address, the payload and message type.
```json
{
  "destination_chain": "Ethereum",
  "destination_address": "0x777d2D82dAb1BF06a4dcf5a3E07057C41100c22D",
  "payload": ...,
  "type": 1
}
```

Upon arrival, Axelar authenticates the message based on channel-id, validators co-sign an approval and relayers relay the approval to Axelar evm gateway.

Axelar provides auto executing service if the smart contract implements the `IAxelarExecutable` interface.
[Developer doc](https://docs.axelar.dev/dev/general-message-passing/gmp-messages) contains instruction for solidity smart contract
