const hre = require("hardhat");

async function main() {
  let SendReceiveFactory = await hre.ethers.getContractFactory("SendReceive");
  let sendreceive = await SendReceiveFactory.deploy(
    "0xe432150cce91c13a887f7D836923d5597adD8E31", // axelar gateway
    "0x24C2b56128fF8E7bFaD578ABefB0fc7Dfa9ba358", // axelar gas service
    "onyx"                                        // chain name
    );

    console.log("SendReceive deployed to: ", sendreceive.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
