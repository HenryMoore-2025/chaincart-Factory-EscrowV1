use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr, // Admin of the factory, typically the marketplace
}

// Storage keys
pub const STATE: Item<State> = Item::new("state");
pub const ESCROWS: Map<&str, Addr> = Map::new("escrows");
pub const TEMP_TRANSACTION_ID: Item<String> = Item::new("temp_transaction_id");

// Configuration for Escrow Contract deployment
pub const ESCROW_CODE_ID: u64 = 1; // Replace with actual code ID after uploading Escrow Contract
pub const REPLY_ID: u64 = 1; // ID for reply handler
