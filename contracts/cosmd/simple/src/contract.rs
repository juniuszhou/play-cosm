use crate::msg::{GreetResp, QueryMsg, ExecuteMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Event,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {

    // define event and attribute
    let events = vec![Event::new("admin_added").add_attribute("addr", "admin")];

    // use events as response
    let resp = Response::new()
            .add_events(events)
            .add_attribute("action", "add_members")
            .add_attribute("added_count", "three");

    Ok(resp)
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
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