'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
} = require('ethers')

const config = require('config');
const [ganache, avax] = config.get('chains');

const MultiSend = require('./artifacts/contracts/MultiSend.sol/MultiSend.json');
const IERC20 = require('./artifacts/@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol/IERC20.json');

const tokenAddr = '0x57F1c63497AEe0bE305B8852b354CEc793da43bB';
const contract = '0xE2cd00e8BBf48AdFb7DF0B00F55260f17127D445';

// args
const destChain = 'osmosis-5';
const destContract = 'osmo1956rjtkm4rh3ncsh5cx0u8552sx80z9ppwmnmucccz3mhs0pglus4rgm8u';
const receiver = ['osmo139a8plum50nhyqvu42papdf6xa9s3nfqdn5lx3', 'osmo1kux208ex604jh4l6js4sap4nuygqw6eakzu9ye'];
const symbol = 'aUSDC';
const amount = 2000000;

(async () => {
    const wallet = new Wallet(
        avax.privateKey,
        new JsonRpcProvider(avax.url),
    );
    
    const multiSend = new Contract(contract, MultiSend.abi, wallet);
    const usda = new Contract(tokenAddr, IERC20.abi, wallet);

    console.log(`wallet has ${(await usda.balanceOf(wallet.address)) / 1e6} ${symbol}`)
    console.log(`gateway is ${(await multiSend.gateway())}`)

    const approveTx = await usda.approve(multiSend.address, amount);
    await approveTx.wait();

    const sendTx = await multiSend.multiSend(destChain, destContract, receiver, symbol, amount);
    const tx = await sendTx.wait();
    
    console.log(`transaction hash is ${tx.transactionHash}`);
})();

