use cosmwasm_std::{Addr, Coin, DepsMut, Response, SubMsg, SubMsgResponse, SubMsgResult};
use cosmwasm_std::{Binary, Env, MessageInfo, Uint128};
use cw20_base::contract::{execute_burn, execute_mint};
use ethabi::ethereum_types::H160;
use ethabi::{decode, encode, ethereum_types::U256, Address, ParamType, Token};
use serde::{Deserialize, Serialize};
use serde_json_wasm::to_string;

use crate::error::ContractError;
use crate::ibc::MsgTransfer;
use crate::state::CONFIG;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct GeneralMessage {
    destination_chain: String,
    destination_address: String,
    payload: Vec<u8>,
    #[serde(rename = "type")]
    type_: i64,
}

pub fn transfer_remote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // burn CW20 token
    execute_burn(deps, env.clone(), info.clone(), amount)?;

    // build payload for token linker
    let addr = match destination_address.parse::<H160>() {
        Ok(address) => Ok(Token::Address(Address::from(address))),
        Err(_) => Err(ContractError::InvalidRecipientAddress { address: destination_address }),
    }?;
    let payload = encode(&[addr, Token::Uint(U256::from(amount.u128()))]);

    let msg = GeneralMessage {
        destination_chain,
        destination_address: config.linker_address,
        payload,
        type_: 1,
    };

    let ibc_transfer = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.channel,
        token: Some(Coin::new(1, "uosmo").into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
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

pub fn execute_from_remote(
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
        &vec![ParamType::String, ParamType::Uint(256)],
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
}
