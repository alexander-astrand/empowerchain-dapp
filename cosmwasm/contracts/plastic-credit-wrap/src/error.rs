use cosmwasm_std::{StdError, StdResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // Add other custom errors here
}

// Converts contract errors into StdResult
pub type ContractResult<T> = StdResult<T>;
