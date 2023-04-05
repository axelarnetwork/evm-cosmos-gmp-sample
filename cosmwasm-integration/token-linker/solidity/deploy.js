'use strict'

const {
    providers: { JsonRpcProvider },
    Wallet,
    ContractFactory,
    constants: { AddressZero },
} = require('ethers')

const config = require('config');
const [ganache, binance] = config.get('chains');

const c = require('./artifacts/contracts/TokenLinker.sol/TokenLinker.json');


(async () => {
    const wallet = new Wallet(
        binance.privateKey,
        new JsonRpcProvider(binance.url),
    );
    const factory = ContractFactory.fromSolidity(c, wallet);

    const contract = await factory.deploy(binance.gateway, AddressZero, "binance", "0xDE41332a508E363079FD6993B81De049cD362B6D");
    const tx = await contract.deployed();

    console.log(`contract deployed on ${tx.address}`);

})();

