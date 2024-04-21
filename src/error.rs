use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("{0}")]
    NonexistentProjectId(#[from] NonexistentProjectIdError),
}

#[derive(Error)]
#[cw_serde]
#[error("Project with id {0} does not exist")]
pub struct NonexistentProjectIdError(pub u128);

pub type ContractResult<T> = Result<T, ContractError>;
