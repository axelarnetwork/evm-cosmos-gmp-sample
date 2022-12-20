'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const MultiSend = require('./artifacts/contracts/MultiSend.sol/MultiSend.json');
const IERC20 = require('./artifacts/@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol/IERC20.json');

const tokenAddr = '0x392B0A115101CC66241bC4180B000EaCEB8e31e3';
const contract = '0x74a010E8B8e6Dc69135DBec8749cEF55d5d09219';

// args
const destChain = 'demo-chain';
const destAddress = 'axelar16rdjmg0ddsy6tg2m945uyj8jnltk4tpw22quxg';
const receiver = ['axelar1cvgeu38h8x0hrqnp39c836fymv7s2an332u6vh', 'axelar17nv682pm98ncvf6x2neqdjx6rd8xpsryppymu7'];
const symbol = 'axlUSDA';
const amount = 1000000;

(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    
    const multiSend = new Contract(contract, MultiSend.abi, wallet);
    const usda = new Contract(tokenAddr, IERC20.abi, wallet);

    console.log(`wallet has ${(await usda.balanceOf(wallet.address)) / 1e6} ${symbol}`)
    console.log(`gateway is ${(await multiSend.gateway())}`)

    const approveTx = await usda.approve(multiSend.address, amount);
    await approveTx.wait();

    const sendTx = await multiSend.multiSend(destChain, destAddress, receiver, symbol, amount);
    const tx = await sendTx.wait();
    
    console.log(`transaction hash is ${tx.transactionHash}`);
})();

