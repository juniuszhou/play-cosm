#[cfg(not(feature = "library"))]

use crate::message::{InstantiateMsg, GreetResp, QueryMsg, UpdateAdmin, ExecuteMsg, QueryAdmin, QueryMyMap};
use crate::state::{ADMIN, MYMAP, MYITEM};
use crate::error::ContractError;

use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Event,
};
use cw_storage_plus::Bound;

pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // from string to address.
    let admin = deps.api.addr_validate(&msg.admin)?;
    // call admin method to save account
    ADMIN.set(deps.branch(), Some(admin))?;

    Ok(Response::new())
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        // handle update admin
        ExecuteMsg::UpdateAdmin {admin} => {
            let admin = deps.api.addr_validate(&admin)?;

            Ok(ADMIN.execute_update_admin(deps, info, Some(admin))?)

        },
        ExecuteMsg::Execute {} => {
            // define event and attribute
            let events = vec![Event::new("admin_added").add_attribute("addr", "admin")];

            // use events as response
            let resp = Response::new()
                    .add_events(events)
                    .add_attribute("action", "add_members")
                    .add_attribute("added_count", "three");

            Ok(resp)
        },
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),

        // call admin member method to get admin account
        QueryAdmin {} => to_binary(&ADMIN.query_admin(deps)?),

        // load value from state with map type
        QueryMyMap { key } => to_binary(&MYMAP.may_load(deps.storage, key)?),

        QueryMyItem {} => to_binary(&MYITEM.may_load(deps.storage)?),
    }
}

mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_query() {
        let resp = query::greet().unwrap();
        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}