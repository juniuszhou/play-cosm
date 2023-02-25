use crate::error::ContractError;
#[cfg(not(feature = "library"))]
use crate::msg::{
    ExecuteMsg, GreetResp, InstantiateMsg, QueryAdmin, QueryMsg, QueryMyMap, UpdateAdmin,
};
use crate::state::ADMIN;
use crate::utils::string_to_eth_address;
use crate::{NATIVE_TOKEN_LOCK, CW20_TOKEN_LOCK};

use cosmwasm_std::{
    entry_point, to_binary, Addr, Attribute, BankMsg, Binary, Deps, DepsMut, Empty, Env, Event,
    MessageInfo, Response, StdResult, SubMsg, WasmMsg,
};
use cw20::Balance;
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
        ExecuteMsg::Lock { balances, receiver } => {
            // check if receiver is valid ethereum address
            string_to_eth_address(&receiver)?;

            let self_address = env.contract.address.clone();

            let admin = ADMIN.get(deps.as_ref()).unwrap().unwrap();

            // let self_address = Addr::try_from(address).map_err(|_| ContractError::Unauthorized {})?;
            // env.contract.address.clone();
            let mut msgs: Vec<SubMsg> = vec![];

            let mut attributes: Vec<Attribute> = vec![];

            for fund in info.funds {
                attributes.push(Attribute::new(fund.denom, fund.amount));
            }

            let native_event = Event::new(NATIVE_TOKEN_LOCK).add_attributes(attributes);

            let mut attributes: Vec<Attribute> = vec![];

            for balance in balances.iter() {
                match balance {
                    // we can't handle native since there is no transfer from method.
                    // we should already get native token from funds.
                    // just add them in event, then relayer can handle it
                    Balance::Native(_) => {

                    }
                    Balance::Cw20(cw20_token) => {
                        let transfer = cw20::Cw20ExecuteMsg::TransferFrom {
                            owner: info.sender.to_string(),
                            recipient: self_address.to_string(),
                            amount: cw20_token.amount,
                        };
                        let message = SubMsg::new(WasmMsg::Execute {
                            contract_addr: cw20_token.address.to_string(),
                            msg: to_binary(&transfer)?,
                            funds: vec![],
                        });

                        msgs.push(message);

                        attributes.push( Attribute::new(cw20_token.address.to_string(), cw20_token.amount.to_string()));
                    }
                }
            }
            let cw20_event = Event::new(CW20_TOKEN_LOCK).add_attributes(attributes);

            Ok(Response::new().add_submessages(msgs).add_events(vec![
                native_event,
                cw20_event,
            ]))
        }
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        // call admin member method to get admin account
        QueryAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
    }
}
