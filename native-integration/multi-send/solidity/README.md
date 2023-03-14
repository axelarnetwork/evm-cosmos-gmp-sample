## Multisend Solidity Contract
Multisend is a sample contract that sends token to multiple cosmos chain addresses.

### Build
```
npm i
npm run buld
```

### Deploy and Interact
Config the private key, and run deploy script to deploy the MultiSend contract
```
node deploy.js
```

The interact script calls `multiSend` to send tokens to remote a cosmos chain
```
node interact.js
```
