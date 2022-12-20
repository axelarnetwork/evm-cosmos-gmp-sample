'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
    utils: {keccak256, toUtf8Bytes, defaultAbiCoder}
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const txHash = '0x13c226c26e426a9d353142af36abf2835bfc9fd3e7d0aea62bb5e31e93bdee24';
const ContractCallWithTokenSig = keccak256(toUtf8Bytes("ContractCallWithToken(address,string,string,bytes32,bytes,string,uint256)"));

(async () => {
    const provider = new JsonRpcProvider(ganache.url);

    const receipt = await provider.getTransactionReceipt(txHash);
    for (let i = 0; i < receipt.logs.length; i++) {
        const log = receipt.logs[i];
        
        if (log.topics[0] === ContractCallWithTokenSig) {
            // get payload
            const args = ['string', 'string', 'bytes', 'string', 'uint256']
            const result = defaultAbiCoder.decode(args, log.data);

            console.log(`event id: ${txHash}-${i}`)
            console.log(`payload: ${result[2].substr(2)}`)
            return 
        }
        
    }
    console.log('ContractCallWithToken not found');
})();
  