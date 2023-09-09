const hre = require("hardhat");
const { ethers } = require("hardhat");
const {
    providers: { JsonRpcProvider },
} = ethers;

async function main() {
  let SendReceiveFactory = await hre.ethers.getContractFactory("SendReceive");
  const contract = SendReceiveFactory.attach(process.env.CONTRACT || "0xcD9ce18C188B0befeE21601beE34be7Ce4cfe255", new JsonRpcProvider(process.env.RPC));

  const message = await contract.storedMessage();
  console.log(`Received message: ${message}`);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
