use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
}

pub type ContractResult<T> = Result<T, ContractError>;
