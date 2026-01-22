#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use ethabi::{decode, ParamType};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, CONFIG_MSG};

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
    }
}

pub fn receive_message(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    source_chain: String,
    source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    // let config = CONFIG.load(deps.storage)?;
    // // Authentication
    // if source_chain != config.source_chain || source_address != config.linker_address
    // || info.sender != config.axelar_gmp_account
    // {
    //     return Err(ContractError::Unauthorized {});
    // }

    // let decoded = decode(
    //     &[ParamType::String, ParamType::Uint(256)],
    //     payload.as_slice(),
    // )
    // .unwrap(); // TODO: handle error

    // // TODO: check if amount and recipient are valid
    // let amount = decoded[1].to_owned().into_uint().unwrap().as_u128();

    // // update the last received message in config
    // let mut config_msg = CONFIG_MSG.load(deps.storage)?;
    // config_msg.last_received_message = format!(
    //     "AT {} RECEIVE: '{}' TOKENs FROM {} ON {}",
    //     env.block.time, amount, source_address, source_chain
    // );

    let mut config_msg = CONFIG_MSG.load(deps.storage)?;
    config_msg.last_received_message = "hahahahahahahaha".to_string();

    // save the config
    CONFIG_MSG.save(deps.storage, &config_msg)?;

    // Base response
    Ok(Response::new().add_attributes([
        ("action", "receive_message"),
        ("source_address", source_address.as_str()),
        ("source_address", source_address.as_str()),
        // ("amount", &amount.to_string()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ConfigMsg {} => to_binary(&CONFIG_MSG.load(deps.storage)?),
    }
}

#[cfg(test)]
mod tests {}
