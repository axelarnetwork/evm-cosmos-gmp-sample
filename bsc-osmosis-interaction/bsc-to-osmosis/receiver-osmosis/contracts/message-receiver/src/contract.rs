#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multi-send";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cfg = Config {
        last_received_message: "".to_string(),
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
        ExecuteMsg::ReceiveMessage {
            destination_chain,
            destination_address,
            message,
        } => receive_message(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
        ExecuteMsg::ReceiveMessageWithToken {
            destination_chain,
            destination_address,
            message,
        } => receive_message_with_token(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
    }
}

pub fn receive_message(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    destination_chain: String,
    destination_contract: String,
    message: String,
) -> Result<Response, ContractError> {
    // update the last received message in config
    let mut config = CONFIG.load(deps.storage)?;
    config.last_received_message = format!(
        "AT {} RECEIVE MESSAGE: '{}' FROM {} ON {}",
        env.block.time, message, destination_contract, destination_chain
    );

    // save the config
    CONFIG.save(deps.storage, &config)?;

    // Base response
    Ok(Response::new().add_attributes([
        ("action", "receive_message"),
        ("destination_chain", destination_chain.as_str()),
        ("destination_contract", destination_contract.as_str()),
        ("message", message.as_str()),
    ]))
}

pub fn receive_message_with_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_contract: String,
    message: String,
) -> Result<Response, ContractError> {
    // get the balance of IBC token sent with the message
    let coin = cw_utils::one_coin(&info).unwrap();

    // update the last received message in config
    let mut config = CONFIG.load(deps.storage)?;
    config.last_received_message = format!(
        "AT {} RECEIVE MESSAGE: '{}' FROM {} ON {} WITH {}{}",
        env.block.time, message, destination_contract, destination_chain, coin.amount, coin.denom
    );

    // save the config
    CONFIG.save(deps.storage, &config)?;

    // Base response
    Ok(Response::new().add_attributes([
        ("action", "receive_message"),
        ("destination_chain", destination_chain.as_str()),
        ("destination_contract", destination_contract.as_str()),
        ("message", message.as_str()),
        ("amount", coin.amount.to_string().as_str()),
        ("denom", coin.denom.as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
    }
}

#[cfg(test)]
mod tests {}
