#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use ethabi::{Address, encode, Token};
use ethabi::ethereum_types::H160;
use serde_json_wasm::to_string;

use crate::error::ContractError;
use crate::ibc::{MsgTransfer};
use crate::types::{Fee, GeneralMessage};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multi-send";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const AXELAR_GATEWAY: &str = "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5";

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
        ExecuteMsg::MultiSend { recipients } => multi_send(deps, env, info, recipients),
        ExecuteMsg::MultiSendToEvm {
            destination_chain,
            destination_address,
            recipients,
            fee
        } => multi_send_to_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            recipients,
            fee
        ),
    }
}

pub fn multi_send(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipients: Vec<String>,
) -> Result<Response, ContractError> {
    let coin = cw_utils::one_coin(&info).unwrap();
    if recipients
        .clone()
        .into_iter()
        .any(|s| deps.api.addr_validate(&s).is_err())
    {
        return Err(ContractError::InvalidRecipient {});
    }

    let amt = coin
        .amount
        .checked_div(Uint128::from(recipients.len() as u64))
        .map_err(StdError::divide_by_zero)?;

    let msgs = recipients.into_iter().map(|s| BankMsg::Send {
        to_address: s.into(),
        amount: coins(amt.u128(), coin.denom.clone()),
    });

    Ok(Response::new().add_messages(msgs))
}



pub fn multi_send_to_evm(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_contract: String,
    recipients: Vec<String>,
    fee: Option<Fee>
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let addresses = recipients
    .into_iter()
    .map(|s| {
        match s.parse::<H160>() {
            Ok(address) => Token::Address(Address::from(address)),
            Err(_) => Err(ContractError::InvalidRecipientAddress { address: s }),
        }
    })
    .collect::<Result<Vec<Token>, ContractError>>()?;
    let payload = encode(&[Token::Array(addresses)]);

    let msg = GeneralMessage {
        destination_chain,
        destination_address: destination_contract.clone(),
        payload,
        type_: 2,
        fee
    };

    let coin = cw_utils::one_coin(&info).unwrap();
    let ibc_transfer = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.channel.to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: AXELAR_GATEWAY.to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&msg).unwrap(),
    };

    // Base response
    let response = Response::new()
        .add_attribute("status", "ibc_message_created")
        .add_attribute("ibc_message", format!("{:?}", ibc_transfer));

    return Ok(response.add_message(ibc_transfer));
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
