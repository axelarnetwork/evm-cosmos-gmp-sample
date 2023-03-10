#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use ethabi::{encode, Token};
use serde::{Deserialize, Serialize};
use serde_json_wasm::to_string;

use crate::error::ContractError;
use crate::ibc::MsgTransfer;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multi-send";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// DON'T CHANGE THIS ADDRESS UNLESS THE AXELAR GATEWAY ADDRESS IS CHANGED
const AXELAR_GATEWAY: &str = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5";

/// DON'T CHANGE THIS STRUCTURE
/// This is the message format for the Axelar Gateway
/// The Axelar Gateway will parse the message and send it to the destination chain
/// @param destination_chain: the chain name to send the message to.
/// @param destination_address: the contract address to send the message to.
/// @param payload: the encoded message will be sent to the destination chain.
///        the payload must be contains the encoded message following Ethereum ABI params standard
///        ref: https://docs.rs/ethabi/18.0.0/ethabi/token/enum.Token.html
/// @param type_: the type of message, 1 for pure message, 2 for message with tokens
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct GeneralMessage {
    destination_chain: String,
    destination_address: String,
    payload: Vec<u8>,
    #[serde(rename = "type")]
    type_: i64,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cfg = Config {
        channel: msg.channel,
    };
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendMessage {
            destination_chain,
            destination_address,
            message,
        } => send_message(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
        ExecuteMsg::SendMessageWithTokens {
            destination_chain,
            destination_address,
            message,
        } => send_message_with_token(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
    }
}

pub fn send_message(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination_chain: String,
    destination_contract: String,
    message: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // encode the message to Ethereum ABI params
    // ref: https://docs.rs/ethabi/18.0.0/ethabi/token/enum.Token.html#variant.String
    let payload = encode(&[Token::String(message)]);

    // create the message to send to the Axelar Gateway
    let msg = GeneralMessage {
        destination_chain,
        destination_address: destination_contract,
        payload,
        type_: 1,
    };

    // create the ibc message
    let ibc_transfer = MsgTransfer {
        source_port: "transfer".to_string(), // What is this?
        source_channel: config.channel,
        token: None, // No token is sent with the message. Error is at here
        sender: env.contract.address.to_string(),
        receiver: AXELAR_GATEWAY.to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&msg).unwrap(),
    };

    // Base response
    Ok(Response::new()
        .add_attributes([
            ("action", "send_message"),
            ("status", "ibc_message_created"),
            ("ibc_message", &format!("{:?}", ibc_transfer)),
        ])
        .add_message(ibc_transfer))
}

pub fn send_message_with_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_contract: String,
    message: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // encode the message to Ethereum ABI params
    // ref: https://docs.rs/ethabi/18.0.0/ethabi/token/enum.Token.html#variant.String
    let payload = encode(&[Token::String(message)]);

    // create the message to send to the Axelar Gateway
    let msg = GeneralMessage {
        destination_chain,
        destination_address: destination_contract,
        payload,
        type_: 2,
    };

    // get the balance of IBC token sent with the message
    let coin = cw_utils::one_coin(&info).unwrap();

    // create the ibc message
    let ibc_transfer = MsgTransfer {
        source_port: "transfer".to_string(), // What is this?
        source_channel: config.channel,
        token: Some(coin.clone().into()),
        sender: env.contract.address.to_string(),
        receiver: AXELAR_GATEWAY.to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&msg).unwrap(),
    };

    // Base response
    Ok(Response::new()
        .add_attributes([
            ("action", "send_message_with_token"),
            ("amount", &coin.amount.to_string()),
            ("denom", &coin.denom),
            ("status", "ibc_message_created"),
            ("ibc_message", &format!("{:?}", ibc_transfer)),
        ])
        .add_message(ibc_transfer))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
