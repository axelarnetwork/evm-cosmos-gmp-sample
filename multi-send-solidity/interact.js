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
const multiSendAddr = '0x3EB10105b5A01CC918e3288A43739968244Ce59E';

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
    const approveTx = await usda.approve(multiSend.address, 1000000);
    await approveTx.wait();
    console.log('approved');

    const accounts = ['cosmos18u4qgduqq0pc62c94fumq4pk66jhn36vez7sf8', 'cosmos18u4qgduqq0pc62c94fumq4pk66jhn36vez7sf8'];
    const sendTx = await multiSend.multiSend('testhub', 'gmp/cosmos1clqaewt8j3errl67dafg4ktjuj8nnvfwn2n2wy', accounts, 'USDA', 1000000);
    const tx = await sendTx.wait();
    console.log(tx);


})();
  