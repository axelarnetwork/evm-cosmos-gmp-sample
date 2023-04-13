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
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cfg = Config {
        channel: msg.channel,
        source_chain: msg.original_chain,
        linker_address: msg.linker_address,
        axelar_gmp_account: msg.axelar_gmp_account,
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
        ExecuteMsg::ExecuteFromRemote {
            source_chain,
            source_address,
            payload,
        } => receive_message(deps, env, info, source_chain, source_address, payload),
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
    info: MessageInfo,
    source_chain: String,
    source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    // Authentication
    if source_chain != config.source_chain
        || source_address != config.linker_address
        || info.sender != config.axelar_gmp_account
    {
        return Err(ContractError::Unauthorized {});
    }

    let decoded = decode(
        &[ParamType::String, ParamType::Uint(256)],
        payload.as_slice(),
    )
    .unwrap(); // TODO: handle error

    // TODO: check if amount and recipient are valid
    let amount = decoded[1].to_owned().into_uint().unwrap().as_u128();

    // call into cw20-base to mint the token, call as self as no one else is allowed
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };
    Ok(execute_mint(
        deps,
        env,
        sub_info,
        decoded[0].to_string(),
        Uint128::from(amount),
    )?)
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
