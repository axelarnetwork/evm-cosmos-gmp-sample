const { ethers } = require("hardhat");

module.exports = async ({
    getNamedAccounts,
    deployments,
}) => {
    const { deploy } = deployments;
    const { deployer } = await getNamedAccounts();

    const AXELAR_GATEWAY = "0x4D147dCb984e6affEEC47e44293DA442580A3Ec0";
    const CHAIN_NAME = "binance";
    const AXELAR_GAS_RECEIVER = ethers.constants.AddressZero;
    const TESTING_AURA_TOKEN = "0xDE41332a508E363079FD6993B81De049cD362B6D";

    await deploy('MessageSender', {
        from: deployer,
        gasLimit: 2000000,
        args: [AXELAR_GATEWAY, AXELAR_GAS_RECEIVER, CHAIN_NAME, TESTING_AURA_TOKEN],
        log: true
    });
};

module.exports.tags = ['contract'];