use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, WasmMsg, Reply, Uint128,
};
use cw_utils::parse_instantiate_response_data;

use crate::error::ContractError;
 use cosmwasm_std::StdError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, EscrowAddressResponse};
use crate::state::{STATE, ESCROWS, TEMP_TRANSACTION_ID, ESCROW_CODE_ID, REPLY_ID};

// Entry point for instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = crate::state::State {
        owner: deps.api.addr_validate(&msg.owner)?,
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// Entry point for execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateEscrow {
            transaction_id,
            buyer,
            seller,
            marketplace,
            required_deposit,
            denom,
            fee_percentage,
        } => execute_create_escrow(
            deps,
            env,
            info,
            transaction_id,
            buyer,
            seller,
            marketplace,
            required_deposit,
            denom,
            fee_percentage,
        ),
    }
}

fn execute_create_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    transaction_id: String,
    buyer: String,
    seller: String,
    marketplace: String,
    required_deposit: Uint128,
    denom: String,
    fee_percentage: u8,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    if ESCROWS.may_load(deps.storage, &transaction_id)?.is_some() {
        return Err(ContractError::TransactionIdExists {});
    }

    // Create instantiation message for the Escrow Contract
    // Note: You'll need to define this message type or import it from the escrow crate
    let instantiate_msg = crate::msg::EscrowInstantiateMsg {
        buyer,
        seller,
        marketplace,
        required_deposit,
        denom,
        fee_percentage,
    };

    // Create a submessage to instantiate the Escrow Contract
    let submsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(env.contract.address.to_string()),
            code_id: ESCROW_CODE_ID,
            msg: to_json_binary(&instantiate_msg)?,
            funds: vec![],
            label: format!("Escrow for {}", transaction_id),
        },
        REPLY_ID,
    );

    // Store transaction ID temporarily for reply handling
    TEMP_TRANSACTION_ID.save(deps.storage, &transaction_id)?;

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attribute("action", "create_escrow")
        .add_attribute("transaction_id", transaction_id))
}

// Entry point for handling replies (e.g., after Escrow Contract instantiation)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID => {
            let reply_data = msg.result.into_result().map_err(StdError::generic_err)?;
            
            // Try to get data from msg_responses first (CosmWasm 2.0+), fallback to data field
            let data = if !reply_data.msg_responses.is_empty() {
                // For CosmWasm 2.0+, extract from msg_responses
                reply_data.msg_responses[0].value.clone()
            } else {
                // Fallback for older versions
                reply_data.data.ok_or_else(|| StdError::generic_err("No instantiate data found"))?
            };
            
            let res = parse_instantiate_response_data(&data)?;
            let contract_address = res.contract_address;
            let transaction_id = TEMP_TRANSACTION_ID.load(deps.storage)?;
            ESCROWS.save(deps.storage, &transaction_id, &deps.api.addr_validate(&contract_address)?)?;
            TEMP_TRANSACTION_ID.remove(deps.storage);
            Ok(Response::new()
                .add_attribute("action", "escrow_created")
                .add_attribute("transaction_id", transaction_id)
                .add_attribute("escrow_address", contract_address))
        }
        _ => Err(ContractError::UnknownReplyId {}),
    }
}

// Entry point for queries
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEscrowAddress { transaction_id } => {
            let addr = ESCROWS.load(deps.storage, &transaction_id)?;
            to_json_binary(&EscrowAddressResponse {
                address: addr.to_string(),
            })
        }
    }
}