const hre = require("hardhat");
const { ethers } = require("hardhat");
const {
    providers: { JsonRpcProvider },
} = ethers;

async function main() {
  let SendReceiveFactory = await hre.ethers.getContractFactory("SendReceive");
  const contract = SendReceiveFactory.attach(process.env.CONTRACT || "0xcD9ce18C188B0befeE21601beE34be7Ce4cfe255", new JsonRpcProvider(process.env.RPC));

  const [sender, message] = await contract.storedMessage().split(",");
  console.log(`Received data:`);
  console.log(`Sender: ${sender}`);
  console.log(`Message: ${message}`);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
