use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Escrow {
    pub buyer: Addr,
    pub seller: Addr,
    pub amount: Uint128,
    pub is_completed: bool,
}

pub const ESCROW: Item<Escrow> = Item::new("escrow");
