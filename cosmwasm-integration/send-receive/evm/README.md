# EVM SendReceive smart contract

This project contains the EVM smart contract that can send and receive message payloads to/from Cosmwasm.

This contract is deployed to: `0xcD9ce18C188B0befeE21601beE34be7Ce4cfe255`
## How to use
User must first create a `.env` file that contains
```
KEY = <private key>
RPC = <chain rpc>
```

### Deploy
SendReceive can be deployed to Fuji Testnet using `npx hardhat run ./scripts/deploy.js --network onyx`.

### Send message
`npx hardhat run ./scripts/send.js --network onyx`

### Check for received message
`npx hardhat run ./scripts/check.js --network onyx`

### Tests
Local tests use a mock gateway/gas service found at `/contracts/Mock/AxelarGatewayGasServiceMock.sol`. This is used to mock out all calls from the SendReceive contract to the Gateway and GasService in testing.

Tests can be run using `npx hardhat test`
