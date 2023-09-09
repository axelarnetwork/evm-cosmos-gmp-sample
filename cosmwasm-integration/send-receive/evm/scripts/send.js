const hre = require("hardhat");
const { ethers } = require("hardhat");
const {
    providers: { JsonRpcProvider },
} = ethers;

async function main() {
  let SendReceiveFactory = await hre.ethers.getContractFactory("SendReceive");
  const contract = SendReceiveFactory.attach(process.env.CONTRACT || "0xcD9ce18C188B0befeE21601beE34be7Ce4cfe255", new JsonRpcProvider(process.env.RPC));

  const message = process.argv[2];

  const tx = await contract.send("provenance", "tp1kaulpuq2rpvz9yr3z74eyjxhu2y4yd546gvtw56urgpe8rwkxn7se9kwyk", message);
  console.log(`Tx hash: ${tx.hash}`);

  await tx.wait();
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
