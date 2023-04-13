use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

use cw_controllers::{AdminError, HookError};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    Hook(#[from] HookError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Message contained duplicate member: {member}")]
    DuplicateMember { member: String },

    #[error("Semver parsing error: {0}")]
    // SemVer(#[from] semver::Error),
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

