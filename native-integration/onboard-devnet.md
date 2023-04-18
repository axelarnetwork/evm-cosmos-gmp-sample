# Onboard Devnet
As the Axelar testnet has a strict onboarding process, we provide a devnet for developers to test their native integration with Axelar.

## Endpoints
|          |       |
| :---:    | :---: |
| Chain ID | devnet-wk | 
| RPC | http://afc5d4a439e4a456bafe34c2d9cd955b-182827533.us-east-2.elb.amazonaws.com:26657 |
| GRPC | http://afc5d4a439e4a456bafe34c2d9cd955b-182827533.us-east-2.elb.amazonaws.com:9090 |
| WS  | ws://afc5d4a439e4a456bafe34c2d9cd955b-182827533.us-east-2.elb.amazonaws.com:26657/websocket |
| Gas Token | uwk|

## Supported Chains
| Chain Name | Gateway Address |
| :---:     | :---: |
| ~~goerli~~    | ~~0xfcc9fb9d2c6404D1C1BEB192020D1e7AC2826c8d~~ |
| avalanche | 0x3724270405e60Fb25f99556Ad2104631f38b9b79 |

Can add more test networks and tokens if needed.

## Supported Tokens
| Chain Name | Symbol | Address |
| :---:     | :---: | :---: |
| goerli    | aUSDC | 0x254d06f33bDc5b8ee05b2ea472107E300226659A |
| avalanche | aUSDC | 0xB366eF4Fa35644c7A10C4Ea90F826562475f66e0 |

## Connect to the Devnet
1. Establiblish an IBC transfer channel between your testing chain and the devnet, and run the relyer. Hermes config for the devnet is as follows:
    ```
    [[chains]]
    id = 'devnet-wk'
    rpc_addr = 'http://afc5d4a439e4a456bafe34c2d9cd955b-182827533.us-east-2.elb.amazonaws.com:26657'
    grpc_addr = 'http://afc5d4a439e4a456bafe34c2d9cd955b-182827533.us-east-2.elb.amazonaws.com:9090'
    websocket_addr = 'ws://afc5d4a439e4a456bafe34c2d9cd955b-182827533.us-east-2.elb.amazonaws.com:26657/websocket'
    rpc_timeout = '10s'
    account_prefix = 'axelar'
    key_name = 'axelar-relayer'
    store_prefix = 'ibc'
    max_gas = 8000000
    gas_price = { price = 0.05, denom = 'uwk' }
    gas_multiplier = 1.1
    max_msg_num = 30
    max_tx_size = 2097152
    clock_drift = '15s'
    trusting_period = '6days'
    trust_threshold = { numerator = '1', denominator = '3' }
    ```
2. Download the v0.33 binary from [releaes](https://github.com/axelarnetwork/axelar-core/releases/tag/v0.33.0)

3. Get gas token; the key holds some gas tokens.You can use the following mnemonic to get the key and process the chain onboard.
    ```
    axelard keys add [key-name] --recover 
    
    stove large toddler vital depth claw flat health lonely welcome link again fade avoid lake grain comic hat tiger wreck all frost sunny still
    ```

4. Register your chain on the devnet
    ````
    axelard tx axelarnet add-cosmos-based-chain [chain-name] [prefix] [transfer/channel-id] --from [key-name]  --gas auto --gas-adjustment 1.5 --gas-prices 0.05uwk --node http://affe36ff88f704aa0b1e22b08e6396c3-417748463.us-east-2.elb.amazonaws.com:26657 --chain-id devnet-wk

    e.g.
    axelard tx axelarnet add-cosmos-based-chain test-chain-1 cosmos transfer/channel-0
    ````
5. Activate your chain
    ````
    axelard tx nexus  activate-chain [chain-name] --from [key-name] --gas auto --gas-adjustment 1.5 --gas-prices 0.05uwk --node http://affe36ff88f704aa0b1e22b08e6396c3-417748463.us-east-2.elb.amazonaws.com:26657 --chain-id devnet-wk
    ````
6. Register asset to your chain
    ````
    axelard tx axelarnet register-asset [chain-name] uausdc --from [key-name] --gas auto --gas-adjustment 1.5 --gas-prices 0.05uwk --node http://affe36ff88f704aa0b1e22b08e6396c3-417748463.us-east-2.elb.amazonaws.com:26657 --chain-id devnet-wk
    ````

## Test Send General Message from Avalance Fuji

This code snippet demonstrates how to send a general message from Avalanche Fuji to your testing environment. If you start with the [dummy handler](./sample-middleware/dummy_handler.go), you should be able to see the message in the log.

```js
'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
    utils: {arrayify, hexlify, hexZeroPad, toUtf8Bytes, concat },
} = require('ethers')

const IAxelarGateway = require('./artifacts/@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol/IAxelarGateway.json');

// args
const destChain = 'local-chain';
const destAddress = 'cosmos1pa662zmlv8mzyeg9vrtaemuus6rjggzrz9t7fr';
// version number indicates message send to native chain 0x00000000
const versionBytes = arrayify(hexZeroPad(hexlify(0), 4));
const payload = hexlify(concat([versionBytes, toUtf8Bytes("hello")]));

(async () => {
    const wallet = new Wallet(
        "cf469f1c4b06a6204bb9f977fa2865271a17a4ed2028ba4c064fea4754e81c83",
        new JsonRpcProvider("https://api.avax-test.network/ext/bc/C/rpc"),
    );
    
    const gateway = new Contract("0x3724270405e60Fb25f99556Ad2104631f38b9b79", IAxelarGateway.abi, wallet);

    const sendTx = await gateway.callContract(
        destChain, 
        destAddress,
        payload
    );
    const tx = await sendTx.wait();
    
    console.log(`sent message to ${destChain}, tx hash ${tx.transactionHash}`);
})();
``` 

## Further Improvement
The current devnet lacks an indexer and explorer, making it difficult to debug issues when messages get stuck. We are actively working on developing a staging network that will include a comprehensive suite of tools to enhance the overall developer experience. 