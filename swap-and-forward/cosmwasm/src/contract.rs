#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{wasm_execute , DepsMut, Env, MessageInfo, Response,  Addr, Coin, Uint128};
use cw2::set_contract_version;
use crosschain_swaps::ExecuteMsg::OsmosisSwap;
use crosschain_swaps::msg::FailedDeliveryAction;
use swaprouter::msg::Slippage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, CONFIG, CHANNEL_MAP};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap-and-forward";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let swap_contract = deps.api.addr_validate(&msg.crosschain_swap_contract)?;

    let config = Config {
        crosschain_swap_contract: swap_contract,
    };
    CONFIG.save(deps.storage, &config)?;

    for (chain, channels) in msg.channels.into_iter() {
        CHANNEL_MAP.save(deps.storage, &chain, &channels)?
    }

    let a = &config.crosschain_swap_contract.into_string();

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("swap_contract", a)
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SwapAndForward {
            dest_chain,
            dest_address,
            swap_amount,
            output_denom,
            slippage,
        } => {
            let coin = cw_utils::one_coin(&info)?;
            execute::swap_and_forward(
                deps, 
                dest_chain, 
                dest_address, 
                swap_amount, 
                coin, 
                output_denom, 
                slippage
            )
        }
    }
}

pub mod execute {
    use std::str::FromStr;

    use cosmwasm_std::Decimal;

    use super::*;

    pub fn swap_and_forward(
        deps: DepsMut,
        dest_chain: String,
        dest_address: Addr,
        swap_amount: u128,
        input_coin: Coin,
        output_denom: String,
        slippage: u8,
    ) -> Result<Response, ContractError> {
        let config = CONFIG.load(deps.storage)?;
        
        let input = Coin::new(swap_amount, input_coin.denom);
        
        let percent = Decimal::from_atomics(slippage, 4).unwrap();
        let s = Slippage::Twap { 
            window_seconds: Some(20), 
            slippage_percentage: percent,
        };    
        
        let swap_msg = OsmosisSwap {
            swap_amount,
            output_denom,
            receiver: dest_address,
            slippage: s,
            next_memo: None,
            on_failed_delivery: FailedDeliveryAction::DoNothing
        };

        let msg = wasm_execute(config.crosschain_swap_contract, &swap_msg, vec![input])?;

        Ok(Response::new().add_message(msg))
    }

}


pub mod query {}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Decimal};
}
