# The receiver contract in EVM

To run the contract, you should install `npm` and `yarn` first.

## How to run
### 1. Install dependencies
```bash
yarn install
```

### 2. Config the environment
- Create `.secret` file in the root directory, and put your private key in it.
- Update the information of EVM chain in `hardhat.config.js`.

### 3. Deploy the contract
```bash
npx hardhat deploy --network <network_name>
```

The `network_name` should be the same as the name of EVM chain in `hardhat.config.js`. For example, the BSC testnet is `testnet`.