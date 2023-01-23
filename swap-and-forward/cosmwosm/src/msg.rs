use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr};

#[cw_serde]
pub struct InstantiateMsg {
    /// This should be an instance of the Osmosis crosschain swap contract
    pub crosschain_swap_contract: String,

    /// These are the channels that will be accepted by the contract. This is
    /// needed to avoid sending packets to addresses not supported by the
    /// receiving chain. The channels are specified as (chain name, [channel_id...])
    pub channels: Vec<(String, Vec<String>)>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SwapAndForward {
        /// Destination chain
        dest_chain: String,
        /// The receiver of final receiver
        dest_address: Addr,
        /// The amount to be swapped
        swap_amount: u128,
        /// The final denom to be received (as represented on osmosis)
        output_denom: String,
        // Slippage in basis point
        slippage: u8,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}
