use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use error::ContractError;

mod contract;
mod msg;
mod state;
mod error;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::InstantiateMsg)
  -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg)
  -> StdResult<Binary>
{
    contract::query(deps, env, msg)
}


const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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



#[cfg(test)]
mod tests {
    use msg::*;
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn greet_query() {
        let mut app = App::default();

        let admin_address = Addr::unchecked("cosmos1address");

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {admin: admin_address.to_string()},
                &[],
                "Contract",
                None,
            )
            .unwrap();
        

        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}