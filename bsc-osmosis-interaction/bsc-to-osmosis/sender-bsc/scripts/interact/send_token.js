'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
} = require('ethers')

// const config = require('config');
// const [ganache, avax, g, biannce] = config.get('chains');

const c = require('../../deployments/testnet/MessageSender.json');


const contract = '0x18e2d2498f084c276abA3680010b209988E1456c';

const IERC20 = require('../../artifacts/@axelar-network/axelar-cgp-solidity/contracts/interfaces/IERC20.sol/IERC20.json');
const tokenAddr = '0xDE41332a508E363079FD6993B81De049cD362B6D';

// args
const destChain = 'osmosis-5';
const destContract = 'osmo1rs93gp7tw3fkls78xdjz09yll6rtv5eag0chtazl8m2hy37xtjpssd6009';
const recipient = 'osmo1uaflg8e46wwtvm0td8mkjeaa0d5s53c9eqk4qg';
const amount = 1000000;

(async () => {
    const wallet = new Wallet(
        "640eda95c742c220925b4650558f0294218f780c426b4b722892f56c3816f4e7",
        new JsonRpcProvider("https://data-seed-prebsc-1-s2.binance.org:8545"),
    );
    
    const tokenLinker = new Contract(contract, c.abi, wallet);

    console.log(`gateway is ${(await tokenLinker.gateway())}`)
    const originalToken = new Contract(tokenAddr, IERC20.abi, wallet);

    console.log(`wallet has ${(await originalToken.balanceOf(wallet.address)) / 1e6}`)

    const approveTx = await originalToken.approve(tokenLinker.address, amount);
    await approveTx.wait();

    const sendTx = await tokenLinker.transferToCosmos(destChain, destContract, recipient, amount);
    const tx = await sendTx.wait();
    
    console.log(`transaction hash is ${tx.transactionHash}`);
})();

