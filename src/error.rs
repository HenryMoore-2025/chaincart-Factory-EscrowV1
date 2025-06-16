use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ParseReply(#[from] ParseReplyError),

    #[error("Unauthorized: only the owner can perform this action")]
    Unauthorized {},

    #[error("Transaction ID already exists")]
    TransactionIdExists {},

    #[error("Unknown reply ID")]
    UnknownReplyId {},
}