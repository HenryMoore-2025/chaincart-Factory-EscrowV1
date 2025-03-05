use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    InitiateEscrow { seller: Addr, amount: Uint128 }, // Buyer initiates escrow
    ReleaseFunds {},                                  // Buyer releases funds to seller
    CancelEscrow {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
