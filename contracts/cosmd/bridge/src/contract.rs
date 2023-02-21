use crate::error::ContractError;
#[cfg(not(feature = "library"))]
use crate::msg::{
    ExecuteMsg, GreetResp, InstantiateMsg, QueryAdmin, QueryMsg, QueryMyMap, UpdateAdmin,
};
use crate::state::ADMIN;
use crate::query::greet;

use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Bound;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // from string to address.
    // let admin = deps.api.addr_validate(info.sender)?;
    // call admin method to save account
    ADMIN.set(deps, Some(info.sender))?;

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // handle update admin
        ExecuteMsg::UpdateAdmin { admin } => {
            let admin = deps.api.addr_validate(&admin)?;

            Ok(ADMIN.execute_update_admin(deps, info, Some(admin))?)
        }
        ExecuteMsg::Execute {} => {
            // define event and attribute
            let events = vec![Event::new("admin_added").add_attribute("addr", "admin")];

            // use events as response
            let resp = Response::new()
                .add_events(events)
                .add_attribute("action", "add_members")
                .add_attribute("added_count", "three");

            Ok(resp)
        }
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&greet()?),

        // call admin member method to get admin account
        QueryAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_query() {
        let resp = greet().unwrap();
        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}
