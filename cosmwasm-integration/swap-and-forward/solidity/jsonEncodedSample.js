'use strict'

const {
    providers: { JsonRpcProvider },
    Contract,
    Wallet,
    utils: {toUtf8Bytes, defaultAbiCoder, hexZeroPad, hexlify},
} = require('ethers')

const config = require('config');
const [ganache] = config.get('chains');

const IAxelarGateway = require('./artifacts/@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IAxelarGateway.sol/IAxelarGateway.json');
const IERC20 = require('./artifacts/@axelar-network/axelar-gmp-sdk-solidity/contracts/interfaces/IERC20.sol/IERC20.json');


const destChain = 'osmosis';
const destAddress = 'osmo1zr7e2zxdeu9nk5j0wwhzfj034xhunx56mqwh7fqf24wdsp2xf5mqpcpjpl';
const symbol = 'axlUSDA';
const amount = '20';
const tokenAddr = '0x392B0A115101CC66241bC4180B000EaCEB8e31e3';

(async () => {
    const wallet = new Wallet(
        ganache.privateKey,
        new JsonRpcProvider(ganache.url),
    );
    
    const gateway = new Contract(ganache.gateway, IAxelarGateway.abi, wallet);
    const usda = new Contract(tokenAddr, IERC20.abi, wallet);

    console.log(`wallet has ${(await usda.balanceOf(wallet.address)) / 1e6} ${symbol}`)
    console.log(`gateway is ${ gateway.address}`)
    
    var data = {
        osmosis_swap: {
            "swap_amount": "20", 
            "output_denom":"ibc/CC6F375AD34C25175AF6F0F49E5F2726DE96C8B49D9D285183DE36F381BE90C6",
            "slippage":{"twap": {"slippage_percentage":"20", "window_seconds": 10}},
            "receiver":"axelar1fzzecwpff2y40x60c902jh8w6s5yfl9snxpg8t",
            "on_failed_delivery": "do_nothing",
            "next_memo":null
          }
      }

    const payload = JSON.stringify(data);

    const approveTx = await usda.approve(ganache.gateway, amount);
    await approveTx.wait();

    const sendTx = await gateway.callContractWithToken(
        destChain, 
        destAddress,
        defaultAbiCoder.encode(
            [ "bytes32", "bytes" ],
            [hexZeroPad(hexlify(2), 32), toUtf8Bytes(payload)]
        ),
        symbol, 
        amount
    );
    const tx = await sendTx.wait();
    
    console.log(`transaction hash is ${tx.transactionHash}`);
})();

