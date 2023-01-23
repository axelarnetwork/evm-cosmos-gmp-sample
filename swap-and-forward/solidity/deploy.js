'use strict'

const {
    providers: { JsonRpcProvider },
    Wallet,
    ContractFactory,
    constants: { AddressZero },
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const swapAndForward = require('./artifacts/contracts/swapAndForward.sol/SwapAndForward.json');


(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    const factory = ContractFactory.fromSolidity(swapAndForward, wallet);

    const contract = await factory.deploy(ganache.gateway, AddressZero, "osmo1ffl5486r44j4ssjcyhxjcuh5z0443k7sfmjejmfs86k32vfwq3zqwklu9u");
    const tx = await contract.deployed();

    console.log(`swap and forward contract deployed on ${tx.address}`);

})();

