const { ethers } = require("hardhat");
const { expect } = require("chai");

describe("SendReceive Test", function () {
  let SendReceiveFactory, AxelarMockFactory, sendReceive, axelarMock;
  let owner, user1;

  const chainName = "local_test";
  
  before(async function () {
    [owner, user1] = await ethers.getSigners();
    SendReceiveFactory = await ethers.getContractFactory("SendReceive");
    AxelarMockFactory = await ethers.getContractFactory("AxelarGatewayGasServiceMock");
  });

  beforeEach(async function() {
    axelarMock = await AxelarMockFactory.deploy();
    sendReceive = await SendReceiveFactory.deploy(axelarMock.address, axelarMock.address, chainName);
  })

  it("Send payload", async function() {
    let fee = ethers.utils.parseEther("1");
    let message = "Hello"; 
    
    // send message to gateway as user1
    await sendReceive.connect(user1).send("destinationChain", "destinationAddress", message, { value: fee });

    // decode payload and verify expected values
    let payload = await axelarMock.storedPayload();
    let version = payload.substring(0, 10);
    expect(version).to.equal("0x00000001"); // Version 1

    let gmpPayload = "0x" + payload.substring(10); // turn string to hex

    // gmpPayload
    let [contractMethod, argumentNameArray, abiTypeArray, argValues] =
      ethers.utils.defaultAbiCoder.decode(["string", "string[]", "string[]", "bytes"], gmpPayload);
    expect(contractMethod).to.equal("receive_message_evm");
    expect(argumentNameArray.toString()).to.equal("source_chain,source_address,payload");
    expect(abiTypeArray.toString()).to.equal("string,string,bytes"); 

    // argValues
    let [chain, sourceAddress, msgPayload] =
    ethers.utils.defaultAbiCoder.decode(["string", "string", "bytes"], argValues);
    expect(chain).to.equal(chainName);
    expect(ethers.utils.getAddress(sourceAddress)).to.equal(sendReceive.address);

    // executeMsgPayload
    let [sender, decoded_message] =
      ethers.utils.defaultAbiCoder.decode(["string", "string"], msgPayload);
    expect(ethers.utils.getAddress(sender)).to.equal(user1.address);
    expect(decoded_message).to.equal(message);
  })

  it("Receive payload", async function() {
    let payload_sender = "sender";
    let payload_message = "hello";

    // generate return payload
    let returnPayload = ethers.utils.defaultAbiCoder.encode(["string", "string"], [payload_sender, payload_message]);

    // call _execute()
    await axelarMock.executeFromGateway(sendReceive.address, "source_chain", "source_address", returnPayload);

    // verify stored message
    let storedMessage = await sendReceive.storedMessage()
    expect(storedMessage.sender).to.equal(payload_sender);
    expect(storedMessage.message).to.equal(payload_message);
  })

  
  it.skip("Decode Utility", async function() {
    // Utility test to decode the payload listed by axelarScan
    // Replace {gmpPayload} with the hex string to be decoded
    let gmpPayload = "0x000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000002b6f736d6f317677306368786d37706d343539756875756d6d6a396e787a7235357161336d743836357135630000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007676f6f6462796500000000000000000000000000000000000000000000000000";
    let [sender, message] = ethers.utils.defaultAbiCoder.decode(["string", "string"], gmpPayload);

    console.log(sender);
    console.log(message);
  })
});