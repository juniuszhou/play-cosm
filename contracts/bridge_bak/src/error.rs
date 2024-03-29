use cosmwasm_std::{OverflowError, StdError};
use cw_controllers::{AdminError, HookError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Hook(#[from] HookError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Message contained duplicate member: {member}")]
    DuplicateMember { member: String },

    #[error("Ethereum address is not valid: {address}")]
    InvalidEthereumAddress { address: String },

    #[error("Token burned not bridge CW20: {address}")]
    BurnNoneBridgeToken { address: String },

    #[error("Failed to create bridge cw20 contract")]
    FailToCreateBridgeCW20Token {},
}
