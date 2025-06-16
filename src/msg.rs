use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateEscrow {
        transaction_id: String,
        buyer: String,
        seller: String,
        marketplace: String,
        required_deposit: Uint128,
        denom: String,
        fee_percentage: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EscrowAddressResponse)]
    GetEscrowAddress { transaction_id: String },
}

#[cw_serde]
pub struct EscrowAddressResponse {
    pub address: String,
}

// Message type for instantiating the escrow contract
#[cw_serde]
pub struct EscrowInstantiateMsg {
    pub buyer: String,
    pub seller: String,
    pub marketplace: String,
    pub required_deposit: Uint128,
    pub denom: String,
    pub fee_percentage: u8,
}