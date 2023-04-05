# Crosschain Token

The Crosschain Token is built on top of the CW20-base and includes `ExecuteFromRemote` and `TransferRemote` functions to facilitate crosschain token transfers.

### Initialization
Crosschain Token
The Crosschain Token is built on top of the CW20-base and includes ExecuteFromRemote and TransferToRemote functions to facilitate crosschain token transfers.

Initialization
To initialize a crosschain token, provide the following parameters:

- original_chain: The chain name of the original token deployed on the EVM chain
- linker_address: The address of the token linker contract deployed on the EVM chain
- axelar_gmp_account: The Axelar GMP account address representation on the Cosmos chain

```bash
# Sample initialization message
MSG='{"name": "Test Aura", "symbol": "TAURA", "decimals": 6, "channel": "channel-1946", "original_chain": "binance", "linker_address": "0x6070819399f94cec4a2aa43306d417957db4e688", "axelar_gmp_account": "osmo1ugjmqpgcw6v3kn82g4zc3xf0n9u4zm7qz8p0f6w083254se74umsempjlt"}'
```

### Execute From Remote
The `ExecuteFromRemote` function can only be called by the Axelar GMP account, which mints tokens for the recipient address.

### Transfer Remote
To transfer a crosschain cw token to another chain, call the TransferToRemote function with the following parameters:

- destination_chain: The chain name of the destination chain
- destination_address: The address on the destination chain
- amount: The amount of tokens to be transferred

```bash
# Sample transfer remote message
MSG='{"transfer_remote": {"destination_chain": "binance", "destination_address": "0x20495d91d19d2967b88835539c8f68aa92265220", "amount": "500000"}}'
```