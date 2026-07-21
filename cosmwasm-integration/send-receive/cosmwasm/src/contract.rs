#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ethabi::{decode, encode, ParamType, Token};
use serde_json_wasm::to_string;

// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:send-receive";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        SendMessageEvm {
            destination_chain,
            destination_address,
            message,
        } => exec::send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
        SendMessageCosmos {
            destination_chain,
            destination_address,
            message,
        } => exec::send_message_cosmos(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
        ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => exec::receive_message_evm(deps, source_chain, source_address, payload),
        ReceiveMessageCosmos {
            sender,
            message
        } => exec::receive_message_cosmos(deps, sender, message),
    }
}

mod exec {
    use super::*;

    // Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
    pub fn send_message_evm(
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        destination_chain: String,
        destination_address: String,
        message: String,
    ) -> Result<Response, ContractError> {
        // Message payload to be received by the destination
        let message_payload = encode(&vec![
            Token::String(info.sender.to_string()),
            Token::String(message),
        ]);

        // {info.funds} used to pay gas. Must only contain 1 token type.
        let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

        let gmp_message: GmpMessage = GmpMessage {
            destination_chain,
            destination_address,
            payload: message_payload.to_vec(),
            type_: 1,
            fee: None,
        };

        let ibc_message = crate::ibc::MsgTransfer {
            source_port: "transfer".to_string(),
            source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
            token: Some(coin.into()),
            sender: env.contract.address.to_string(),
            receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
                .to_string(),
            timeout_height: None,
            timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
            memo: to_string(&gmp_message).unwrap(),
        };

        Ok(Response::new().add_message(ibc_message))
    }

    // Sends a message via Axelar GMP to the other cosmos chains
    // only difference is how the {message_payload} is constructed
    pub fn send_message_cosmos(
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        destination_chain: String,
        destination_address: String,
        message: String,
    ) -> Result<Response, ContractError> {
        // Construct contract call
        let contract_call = serde_json_wasm::to_string(&ExecuteMsg::ReceiveMessageCosmos { sender: info.sender.to_string(), message })
            .expect("Failed to serialize struct to JSON");
        let utf8_bytes = contract_call.as_bytes();
        let utf8_vec = utf8_bytes.to_owned();
        // prepend 4 bytes to indicate the payload verison
        let mut message_payload: Vec<u8> = vec![0, 0, 0, 2];
        message_payload.extend(utf8_vec);

        let gmp_message: GmpMessage = GmpMessage {
            destination_chain,
            destination_address,
            payload: message_payload.to_vec(),
            type_: 1,
            fee: None,
        };

        // info.funds used to pay gas. Must only contain 1 token type.
        let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

        let ibc_message = crate::ibc::MsgTransfer {
            source_port: "transfer".to_string(),
            source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
            token: Some(coin.into()),
            sender: env.contract.address.to_string(),
            receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
                .to_string(),
            timeout_height: None,
            timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
            memo: to_string(&gmp_message).unwrap(),
        };

        Ok(Response::new().add_message(ibc_message))
    }

    pub fn receive_message_evm(
        deps: DepsMut,
        _source_chain: String,
        _source_address: String,
        payload: Binary,
    ) -> Result<Response, ContractError> {
        // Demo only — this shouldn't be used as-is in production: it does NOT authorize the sender.
        // `execute` is a public entry point, so anyone on this chain can call ReceiveMessageEvm with
        // forged source_chain/source_address and overwrite state.
        //
        // To make this production-safe, authorize the sender exactly like token-linker's
        // `execute_from_remote` in this repo. It is mechanical — add config + one guard:
        //
        //   1. state.rs — store the trusted origin, set once at instantiate:
        //        #[cw_serde]
        //        pub struct Config {
        //            pub axelar_gmp_account: String, // Axelar's GMP account derived address on THIS
        //                                            // chain — this is what `info.sender` will equal
        //            pub source_chain: String,       // trusted origin, e.g. "ethereum"
        //            pub source_address: String,     // trusted EVM sender contract, e.g. "0xAbc..."
        //        }
        //        pub const CONFIG: Item<Config> = Item::new("config");
        //      Populate it in instantiate() from InstantiateMsg (currently empty), and thread
        //      `info: MessageInfo` into this handler — execute()'s dispatch currently drops it.
        //
        //   2. Guard at the top of this fn (source_chain/source_address are already passed in;
        //      drop the leading underscores from the params once you use them):
        //        let config = CONFIG.load(deps.storage)?;
        //        if source_chain != config.source_chain
        //            || source_address != config.source_address
        //            || info.sender != config.axelar_gmp_account
        //        {
        //            return Err(ContractError::Unauthorized {});
        //        }
        //
        // Why all three: info.sender is stamped by the ibc-hooks middleware and cannot be spoofed by
        // a direct caller, so it proves the call truly arrived via Axelar GMP over the expected
        // channel; source_chain/source_address prove which EVM contract sent it. The source fields
        // alone are worthless (a direct caller forges them) — the info.sender check is what makes
        // them trustworthy.
        // decode the payload
        // executeMsgPayload: [sender, message]
        let decoded = decode(
            &vec![ParamType::String, ParamType::String],
            payload.as_slice(),
        )
        .unwrap();

        // store message
        STORED_MESSAGE.save(
            deps.storage,
            &Message {
                sender: decoded[0].to_string(),
                message: decoded[1].to_string(),
            },
        )?;

        Ok(Response::new())
    }

    pub fn receive_message_cosmos(deps: DepsMut, sender: String, message: String) -> Result<Response, ContractError> {
        // store message
        STORED_MESSAGE.save(
            deps.storage,
            &Message {
                sender,
                message
            },
        )?;

        Ok(Response::new())
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        GetStoredMessage {} => to_binary(&query::get_stored_message(deps)?),
    }
}

mod query {
    use super::*;

    pub fn get_stored_message(deps: Deps) -> StdResult<GetStoredMessageResp> {
        let message = STORED_MESSAGE.may_load(deps.storage).unwrap().unwrap();
        let resp = GetStoredMessageResp {
            sender: message.sender,
            message: message.message,
        };
        Ok(resp)
    }
}
