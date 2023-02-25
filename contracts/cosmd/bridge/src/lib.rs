use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use error::ContractError;

mod contract;
pub mod error;
pub mod msg;
mod response;
mod state;
mod utils;

#[cfg(test)]
mod tests;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const NATIVE_TOKEN_LOCK: &str = "native_token_lock";
const CW20_TOKEN_LOCK: &str = "cw20_token_lock";

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
