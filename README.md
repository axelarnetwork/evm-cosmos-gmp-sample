## EVM <-> Cosmos Generam Message Passing Demo
This repo contains an example that sends message between EVM chains and cosmos chains.

### EVM
Developer needs to deploy a smart contract that call Axelar gateway to send cross chain message, and decode message upon receiving the payload from remote chain. `multi-send-solidty` contains a sample contract.

### Cosmos Native Integration
A Cosmos chain can integrate cross chain message passing by adding a IBC middleware to decode the paylod, and creating functionality to send cross chain message. `multi-send-cosmos-native` contains a sample middleware and a tx to encode and send payload. 