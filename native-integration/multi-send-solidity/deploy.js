'use strict'

const {
    providers: { JsonRpcProvider },
    Wallet,
    ContractFactory,
    constants: { AddressZero },
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const MultiSend = require('./artifacts/contracts/MultiSend.sol/MultiSend.json');


(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    const factory = ContractFactory.fromSolidity(MultiSend, wallet);

    const contract = await factory.deploy(ganache.gateway, AddressZero)
    const tx = await contract.deployed();

    console.log(`multi send contract deployed on ${tx.address}`);

})();

