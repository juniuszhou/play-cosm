use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_binary
};
use error::ContractError;
use semver::Version;

use cw2::{get_contract_version, set_contract_version};

mod contract;
mod message;
mod state;
mod error;

#[cfg(test)]
mod test;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: message::InstantiateMsg)
  -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: message::ExecuteMsg) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: message::QueryMsg)
  -> StdResult<Binary>
{
    // match msg.clone() {
    //     Empty => contract::query(deps, env, Empty),
    //     _ => contract::query(deps, env, msg),
    //
    // }
    let item = state::MYITEM.load(deps.storage)?;
    to_binary(&item)
    // contract::query(deps, env, msg)
}


const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: Empty) -> Result<Response, ContractError> {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    if storage_version < version {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // If state structure changed in any contract version in the way migration is needed, it
        // should occur here
    }
    Ok(Response::default())
}
