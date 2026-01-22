# The sender contract in osmosis

To run the contract, you should install:
- [beaker](https://github.com/osmosis-labs/beaker)
- [nodejs](https://nodejs.org/en/download/)

## How to run
### 1. Install dependencies
```bash
npm install
```

### 2. Compile the contract
```bash
beaker wasm build
```

### 3. Deploy the contract
```bash
node ./scripts/0_contract_setup.js multi_send
```

### 4. Run the test
```bash
node ./scripts/1_send_message.js <contract_address> <message>
```