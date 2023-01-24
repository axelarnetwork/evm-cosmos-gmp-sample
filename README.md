## EVM <-> Cosmos Generam Message Passing Demo
Do not follow the instuctions atm, we are updating docs, the devnet and setting up relayers, execution services, etc. We will update soon.
You could still check the provided samples.

This repo contains examples for both native and wasm integration.
- multi send is a native integration example, it sends token to multiple recipients from evm chain to a cosmos chain.
- swap and forward sends token and calls a cosmwasm swap contract on Osmosis

### EVM
Developer needs to deploy a smart contract that calls Axelar gateway to send cross-chain message,
and decode message upon receiving the payload from remote chain.

[MultiSend.sol](./multi-send-solidity/contracts/MultiSend.sol) is a sample contract.

[deploy.js](./multi-send-solidity/deploy.js) and [interact.js](./multi-send-solidity/interact.js) are scripts to deploy and interact with the sample contract.

### Cosmos Native Integration
A Cosmos chain can integrate cross chain message passing by adding an [IBC middleware](./multi-send-cosmos-native/gmpdemo/ibc_middleware.go),
and a handler that implements [GeneralMessageHandler](./multi-send-cosmos-native/gmpdemo/gmp_handler.go#L33) interface to decode the paylod. [MultiSendHandler](./multi-send-cosmos-native/gmpdemo/keeper/multi_send_handler.go#L27) is a sample implementation.

[MultiSend](./multi-send-cosmos-native/gmpdemo/keeper/msg_server.go) is a sample transaction that sends payload to an EVM chain.

### Testing Environment
We spinned up a devnet that supports message passing for testing purpose. The devnet currently supports
| Chain | Gateway Address    | RPC |
| :---:   | :---:|:---:|
|  ganache-0 |  0xE720c5C38028Ca08DA47E179162Eca2DD255B6eC    | http://a087b4719fc8944a0952490cf1020812-853925870.us-east-2.elb.amazonaws.com:7545|
| will add more chains ...|...|...|

| Chain | Token| Token Address|
| :---:   | :---:| :---:|
|  ganache-0 |  axlUSDA    | 0x392B0A115101CC66241bC4180B000EaCEB8e31e3
| ...|...|...|

### Setup relayer to the devnet
1. Set up relayer, create an IBC channel between your local/test chain and the devnet. [sample hermes config](./devnet-vx/sample-hermes-config.toml) for the devnet

5. Whitelist the chain on axelar devnet. (Reach out to us)
### How to send cross-chain message with token from a EVM chain to a Cosmos chain
1. Deploy your contract, use [sample contract](./multi-send-solidity/contracts/MultiSend.sol) as reference

2. Call the method that sends cross-chain message with token

3. Confirm the transaction on axelar network
    ```
    axelard tx evm confirm-gateway-tx [evm-chain-name] [tx hash] --from wallet $TXFLAG
    ```
    You can query the event status, wait the `status` field becomes `STATUS_COMPLETED`. (about ~10 seconds)
    ```
    axelard q evm event [evm-chain-name] [event-id] $NODE
    
    ```
    **[sample script](./multi-send-solidity/getEventIdAndPayload.js) to get event id and payload**

4. Execute payload on axelar network
    ```
    axelard tx axelarnet execute-general-message-with-token [dest-chain] [event-id] [payload] --from wallet $TXFLAG
    ```
   The payload is sent via IBC transfer packet `memo` field

5. Upon arrival, the payload is passed to `GeneralMessageHandler`

#### Send cross-chain message with token from a cosmos chain to a EVM chain.

1. ABI encode the payload and send the payload via IBC transfer. use [this sample](./multi-send-cosmos-native/gmpdemo/keeper/msg_server.go#L33) as reference
2. Upon arrival, you should be able to query the pending message that needs to be signed by validators
    ```
    axelard q evm  pending-commands [dest-evm-chain] $NODE
    ```
3. Ask validators to sign in order to approve the message passing.
    ```
    axelard tx evm sign-commands [dest-evm-chain] --from wallet $TXFLAG
    ```
    Find the `batchedCommandID` field from transacton response
3. Wait for the batched status changes to `BATCHED_COMMANDS_STATUS_SIGNED`
    ```
    axelard q evm  batched-commands [dest-evm-chain] [batchedCommandID] $NODE
    ```
4. Get the `execute_data` field from the query response above and broadcast the signed data to axelar gateway on the EVM chain.
Check [sendExecuteData.js](./multi-send-solidity/sendExecuteData.js), it helps send the exeute data. The gateway should emit a `ContractCallApprovedWithMint` event

5. Call `executeWithToken` function from your contract to execute the payload. [executeWithToken.js](./multi-send-solidity/executeWithToken.js) shows how to execurte the payload.
