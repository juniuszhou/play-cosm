use crate::error::ContractError;
// #[cfg(not(feature = "library"))]
use crate::msg::{EthClaim, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{add_bridge_token, get_cw20_via_eth_address, ADMIN};
use crate::utils::{create_cw20_contract, is_bridge_token, string_to_eth_address};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use crate::{CW20_TOKEN_BURN, CW20_TOKEN_LOCK, NATIVE_TOKEN_LOCK};
use cw2::set_contract_version;

use crate::msg::{CosmosToken, QueryAdmin};
use cosmwasm_std::{
    entry_point, to_binary, Attribute, BankMsg, Binary, CodeInfoResponse, Deps, DepsMut, Env,
    Event, MessageInfo, Response, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw20::{Balance, Cw20CoinVerified};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // from string to address.
    // let admin = deps.api.addr_validate(info.sender)?;
    // call admin method to save account
    ADMIN.set(deps, Some(info.sender))?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
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
        ExecuteMsg::Burn { balances, receiver } => {
            execute_burn(deps, env, info, &balances, &receiver)
        }
        ExecuteMsg::Lock { balances, receiver } => {
            execute_lock(deps, env, info, &balances, &receiver)
        }
        ExecuteMsg::BridgeClaim { claims } => execute_bridge_claim(deps, env, info, &claims),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        // call admin member method to get admin account
        QueryAdmin {} => to_binary(&ADMIN.query_admin(deps)?),
    }
}

fn execute_bridge_claim(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    claims: &Vec<EthClaim>,
) -> Result<Response, ContractError> {
    let code_id = 0_u64;
    let mut messages: Vec<SubMsg> = vec![];

    for claim in claims.iter() {
        // check claim hash if it is processed
        // claim.claim_hash
        string_to_eth_address(&claim.token_address)?;
        match &claim.cosmos_token {
            // eth burn case
            Some(token) => {
                match token {
                    CosmosToken::Native(native) => {
                        // create bank transfer message
                        let transfer = BankMsg::Send {
                            to_address: claim.receiver.to_string(),
                            amount: vec![native.to_owned()],
                        };
                        messages.push(SubMsg::new(transfer));
                    }
                    CosmosToken::CW20(cw20) => {
                        let transfer = cw20::Cw20ExecuteMsg::Transfer {
                            recipient: claim.receiver.to_string(),
                            amount: Uint128::from(cw20.amount),
                        };

                        let message = SubMsg::new(WasmMsg::Execute {
                            contract_addr: cw20.address.to_string(),
                            msg: to_binary(&transfer)?,
                            funds: vec![],
                        });
                        messages.push(message);
                    }
                }
            }
            // eth lock case
            None => {
                // query the cw20 for eth token address
                // create new cw20 if not created before
                // mint token for receiver
                let cw20_address = match get_cw20_via_eth_address(
                    deps.storage,
                    &claim.token_address.to_string(),
                ) {
                    Some(addr) => addr,
                    None => {
                        let CodeInfoResponse { checksum, .. } =
                            deps.querier.query_wasm_code_info(code_id)?;
                        let (address, message) = create_cw20_contract(
                            deps.storage,
                            deps.api,
                            // deps.storage,
                            &env.contract.address.to_string(),
                            code_id,
                            checksum,
                        )
                        .map_err(|_| ContractError::FailToCreateBridgeCW20Token {})?;
                        messages.push(SubMsg::new(message));
                        // add new created cw20 token into state
                        add_bridge_token(deps.storage, &claim.token_address, &address)?;
                        address
                    }
                };

                let mint = cw20::Cw20ExecuteMsg::Mint {
                    recipient: claim.receiver.to_string(),
                    amount: Uint128::from(claim.amount),
                };

                let message = SubMsg::new(WasmMsg::Execute {
                    contract_addr: cw20_address.to_string(),
                    msg: to_binary(&mint)?,
                    funds: vec![],
                });
                messages.push(message);
            }
        }
    }
    Ok(Response::new().add_submessages(messages))
}

fn execute_burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    balances: &Vec<Cw20CoinVerified>,
    receiver: &str,
) -> Result<Response, ContractError> {
    string_to_eth_address(&receiver)?;

    let mut messages: Vec<SubMsg> = vec![];
    let mut attributes: Vec<Attribute> = vec![];

    for balance in balances.iter() {
        if !is_bridge_token(deps.storage, &balance.address) {
            return Err(ContractError::BurnNoneBridgeToken {
                address: balance.address.to_string(),
            });
        }

        let burn = cw20::Cw20ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount: Uint128::from(balance.amount),
        };

        let message = SubMsg::new(WasmMsg::Execute {
            contract_addr: balance.address.to_string(),
            msg: to_binary(&burn)?,
            funds: vec![],
        });
        messages.push(message);
        attributes.push(Attribute::new(
            balance.address.to_string(),
            balance.amount.to_string(),
        ));
    }

    let burn_event = Event::new(CW20_TOKEN_BURN).add_attributes(attributes);

    Ok(Response::new()
        .add_submessages(messages)
        .add_event(burn_event))
}

fn execute_lock(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    balances: &Vec<Balance>,
    receiver: &str,
) -> Result<Response, ContractError> {
    // check if receiver is valid ethereum address
    string_to_eth_address(&receiver)?;

    let self_address = env.contract.address.clone();

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
            Balance::Native(_) => {}
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

                attributes.push(Attribute::new(
                    cw20_token.address.to_string(),
                    cw20_token.amount.to_string(),
                ));
            }
        }
    }
    let cw20_event = Event::new(CW20_TOKEN_LOCK).add_attributes(attributes);

    Ok(Response::new()
        .add_submessages(msgs)
        .add_events(vec![native_event, cw20_event]))
}
