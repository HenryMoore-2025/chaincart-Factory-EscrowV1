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
    InitiateEscrow { buyer: Addr, seller: Addr, amount: Uint128 },
    ConfirmEscrow {},
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::InitiateEscrow { buyer, seller, amount } => {
            initiate_escrow(deps, info, buyer, seller, amount)
        }
        ExecuteMsg::ConfirmEscrow {} => confirm_escrow(deps, info),
        ExecuteMsg::CancelEscrow {} => cancel_escrow(deps, info),
    }
}

fn initiate_escrow(
    deps: DepsMut,
    _info: MessageInfo,
    buyer: Addr,
    seller: Addr,
    amount: Uint128,
) -> StdResult<Response> {
    let escrow = Escrow {
        buyer,
        seller,
        amount,
        is_completed: false,
    };
    ESCROW.save(deps.storage, &escrow)?;
    Ok(Response::new().add_attribute("action", "initiate_escrow"))
}

fn confirm_escrow(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut escrow = ESCROW.load(deps.storage)?;
    if info.sender != escrow.seller {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized"));
    }
    escrow.is_completed = true;
    ESCROW.save(deps.storage, &escrow)?;
    Ok(Response::new().add_attribute("action", "confirm_escrow"))
}

fn cancel_escrow(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let escrow = ESCROW.load(deps.storage)?;
    if info.sender != escrow.buyer {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized"));
    }
    ESCROW.remove(deps.storage);
    Ok(Response::new().add_attribute("action", "cancel_escrow"))
}