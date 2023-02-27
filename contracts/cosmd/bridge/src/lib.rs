use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
// use error::ContractError;

// never put contract mod in lib. otherwise it can't pass cargo wasm
// there are conflicts about instantiate/query/execute with other cw20 lib
pub mod contract;
pub mod error;
pub mod msg;
mod response;
mod state;
mod utils;

#[cfg(test)]
mod tests;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const NATIVE_TOKEN_LOCK: &str = "native_token_lock";
const CW20_TOKEN_LOCK: &str = "cw20_token_lock";
const CW20_TOKEN_BURN: &str = "cw20_token_burn";

// #[entry_point]
// pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
//     let version: Version = CONTRACT_VERSION.parse()?;
//     let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

//     if storage_version < version {
//         set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

//         // If state structure changed in any contract version in the way migration is needed, it
//         // should occur here
//     }
//     Ok(Response::default())
// }
