'use strict'

const {
    providers: { JsonRpcProvider },
    Wallet,
    ContractFactory,
    constants: { AddressZero },
} = require('ethers')

const config = require('config');
const [ganache,avax] = config.get('chains');

const c = require('./artifacts/contracts/MultiSend.sol/MultiSend.json');

(async () => {
    const wallet = new Wallet(
        avax.privateKey,
        new JsonRpcProvider(avax.url),
    );
    const factory = ContractFactory.fromSolidity(c, wallet);

    const contract = await factory.deploy(avax.gateway, AddressZero);
    const tx = await contract.deployed();

    console.log(`contract deployed on ${tx.address}`);

})();

