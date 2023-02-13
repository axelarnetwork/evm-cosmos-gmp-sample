'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
    utils: {arrayify, keccak256,formatBytes32String}
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const MultiSend = require('./artifacts/contracts/MultiSend.sol/MultiSend.json');


const contract = '0x74a010E8B8e6Dc69135DBec8749cEF55d5d09219';
const commandID = '0xf54fa9b246b504bcb65514bcad874391e524a2b25a265ca93ed2e9b49a42205b';
const payload = '0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000100000000000000000000000068b93045fe7d8794a7caf327e7f855cd6cd03bb8';

(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    
    
    const multiSend = new Contract(contract, MultiSend.abi, wallet);

    // execute
    const bz = arrayify(payload);
        
    const executeTx = await multiSend.executeWithToken(
        arrayify(commandID),
        "demo-chain",
        "axelar17nv682pm98ncvf6x2neqdjx6rd8xpsryppymu7",
        arrayify(payload),
        "axlUSDA",
        10
    );

    const tx = await executeTx.wait(1);
    console.log(tx);
})();
  