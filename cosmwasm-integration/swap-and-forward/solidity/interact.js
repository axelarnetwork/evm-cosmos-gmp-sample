'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const SwapAndForward = require('./artifacts/contracts/swapAndForward.sol/SwapAndForward.json');
const IERC20 = require('./artifacts/@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol/IERC20.json');
// const IMintable = require('./MintableCappedERC20.json');

const tokenAddr = '0x392B0A115101CC66241bC4180B000EaCEB8e31e3';
const swapAndForwardContract = '0x0E2fd45c0C7f574F9de34b0632a22d81BC210094';

// args
const destChain = 'axelar';
const destAddress = 'axelar1alc0ds3xldwfnena2fs9na3lvjtwrr6246c5jv';
const symbol = 'axlUSDA';
const amount = '10';

(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    
    const swapAndForward = new Contract(swapAndForwardContract, SwapAndForward.abi, wallet);
    const usda = new Contract(tokenAddr, IERC20.abi, wallet);

    console.log(`wallet has ${(await usda.balanceOf(wallet.address)) / 1e6} ${symbol}`)
    console.log(`gateway is ${(await swapAndForward.gateway())}`)
    console.log(`swap contract is ${(await swapAndForward.swapContract())}`);


    const approveTx = await usda.approve(swapAndForward.address, amount);
    await approveTx.wait();

    const sendTx = await swapAndForward.swapAndForward(
        destChain, 
        destAddress, 
        symbol, 
        amount, 
        "ibc/CC6F375AD34C25175AF6F0F49E5F2726DE96C8B49D9D285183DE36F381BE90C6", 
        20
    );
    const tx = await sendTx.wait();
    
    console.log(`transaction hash is ${tx.transactionHash}`);
})();

