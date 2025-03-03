use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    InitiateEscrow { seller: Addr, amount: Uint128 }, // Buyer initiates escrow
    ReleaseFunds {}, // Buyer releases funds to seller
    CancelEscrow {},
}



pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::InitiateEscrow { seller, amount } => {
            initiate_escrow(deps, info, seller, amount)
        }
        ExecuteMsg::ReleaseFunds {} => release_funds(deps, info, env),
        ExecuteMsg::CancelEscrow {} => cancel_escrow(deps, info, env),
    }
}




fn initiate_escrow(
    deps: DepsMut,
    info: MessageInfo,
    seller: Addr,
    amount: Uint128,
) -> StdResult<Response> {
    // Ensure the sender is the buyer (no need to specify buyer in the message)
    let buyer = info.sender;

    // Verify that the correct amount of funds has been sent
    if info.funds.len() != 1 || info.funds[0].amount != amount || info.funds[0].denom != "ujuno" {
        return Err(cosmwasm_std::StdError::generic_err("Incorrect funds sent"));
    }

    // Create a new escrow
    let escrow = Escrow {
        buyer,
        seller,
        amount,
        is_completed: false,
    };

    // Save the escrow to storage
    ESCROW.save(deps.storage, &escrow)?;

    Ok(Response::new().add_attribute("action", "initiate_escrow"))
}

fn release_funds(deps: DepsMut, info: MessageInfo, env: Env) -> StdResult<Response> {
    // Load the escrow from storage
    let escrow = ESCROW.load(deps.storage)?;

    // Ensure the sender is the buyer
    if info.sender != escrow.buyer {
        return Err(cosmwasm_std::StdError::generic_err("Only the buyer can release funds"));
    }

    // Ensure the escrow is not already completed
    if escrow.is_completed {
        return Err(cosmwasm_std::StdError::generic_err("Escrow is already completed"));
    }

    // Transfer funds to the seller
    let transfer_msg = cosmwasm_std::BankMsg::Send {
        to_address: escrow.seller.to_string(),
        amount: vec![cosmwasm_std::Coin {
            denom: "ujuno".to_string(),
            amount: escrow.amount,
        }],
    };

    // Mark the escrow as completed
    ESCROW.save(deps.storage, &Escrow {
        is_completed: true,
        ..escrow
    })?;

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "release_funds"))
}



fn cancel_escrow(deps: DepsMut, info: MessageInfo, env: Env) -> StdResult<Response> {
    // Load the escrow from storage
    let escrow = ESCROW.load(deps.storage)?;

    // Ensure the sender is the buyer
    if info.sender != escrow.buyer {
        return Err(cosmwasm_std::StdError::generic_err("Only the buyer can cancel the escrow"));
    }

    // Ensure the escrow is not already completed
    if escrow.is_completed {
        return Err(cosmwasm_std::StdError::generic_err("Escrow is already completed"));
    }

    // Refund funds to the buyer
    let transfer_msg = cosmwasm_std::BankMsg::Send {
        to_address: escrow.buyer.to_string(),
        amount: vec![cosmwasm_std::Coin {
            denom: "ujuno".to_string(),
            amount: escrow.amount,
        }],
    };

    // Remove the escrow from storage
    ESCROW.remove(deps.storage);

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "cancel_escrow"))
}