const hre = require("hardhat");

async function main() {
  let SendReceiveFactory = await hre.ethers.getContractFactory("SendReceive");
  let sendreceive = await SendReceiveFactory.deploy(
    "0xC249632c2D40b9001FE907806902f63038B737Ab", // axelar gateway
    "0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6", // axelar gas service
    "Avalanche"                                   // fuji chain name
    );
  
    console.log("SendReceive deployed to: ", sendreceive.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
