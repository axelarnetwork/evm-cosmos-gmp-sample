'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
} = require('ethers')

const config = require('config');
const [ganache, binance] = config.get('chains');

const c = require('./artifacts/contracts/TokenLinker.sol/TokenLinker.json');


const contract = '0x6070819399F94Cec4a2aa43306d417957db4E688';

const IERC20 = require('./artifacts/@axelar-network/axelar-cgp-solidity/contracts/interfaces/IERC20.sol/IERC20.json');
const tokenAddr = '0xDE41332a508E363079FD6993B81De049cD362B6D';

// args
const destChain = 'osmosis-5';
const destContract = 'osmo139lnv9h82n5yc4wm877reptryr89fwzep9kqkfch7zwyx947p5rsse2glt';
const recipient = 'osmo1hrhv7xa8ejnk0k6e2kyn62fjjslme8tku28j2f';
const amount = 10000000;

(async () => {
    const wallet = new Wallet(
        binance.privateKey,
        new JsonRpcProvider(binance.url),
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

