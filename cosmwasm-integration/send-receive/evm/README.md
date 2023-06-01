# EVM SendReceive smart contract

This project contains the EVM smart contract that can send and receive message payloads to/from Cosmwasm.

This contract is deployed to Fuji Testnet: https://testnet.snowtrace.io/address/0xa88B3843E895988D423efFc4Ecc2E982f398a3Ab#code
## How to use
User must first create a `.env` file that contains 
```
DEPLOYER_PRIV_KEY = <deployer_wallet_private_key>
FUJI_API_KEY = <infura_api_key_for_fuji_testnet>
```

### Deploy
SendReceive can be deployed to Fuji Testnet using `npx hardhat run ./scripts/deploy.js --network fuji`.

### Tests
Local tests use a mock gateway/gas service found at `/contracts/Mock/AxelarGatewayGasServiceMock.sol`. This is used to mock out all calls from the SendReceive contract to the Gateway and GasService in testing.

Tests can be run using `npx hardhat test`


