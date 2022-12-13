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

const axlUSDAAddr = '0x392B0A115101CC66241bC4180B000EaCEB8e31e3';
const multiSendAddr = '0x74a010E8B8e6Dc69135DBec8749cEF55d5d09219';

// args
const destChain = 'testhub';
const destAddress = 'cosmos1clqaewt8j3errl67dafg4ktjuj8nnvfwn2n2wy';
const receiver = ['cosmos18u4qgduqq0pc62c94fumq4pk66jhn36vez7sf8', 'cosmos18u4qgduqq0pc62c94fumq4pk66jhn36vez7sf8'];
const symbol = 'axlUSDA';
const amount = 1000000;

(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    
    const multiSend = new Contract(multiSendAddr, MultiSend.abi, wallet);
    const usda = new Contract(axlUSDAAddr, IERC20.abi, wallet);

    console.log(`wallet has ${(await usda.balanceOf(wallet.address)) / 1e6} axlUSDA`)
    console.log(`gateway is ${(await multiSend.gateway())}`)

    // multi send
    const approveTx = await usda.approve(multiSend.address, amount);
    await approveTx.wait();
    console.log('approved');

    const sendTx = await multiSend.multiSend(destChain, destAddress, receiver, symbol, amount);
    const tx = await sendTx.wait();
    console.log(tx.transactionHash);
})();
  