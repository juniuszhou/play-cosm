use crate::state::{get_cw20_via_eth_address, add_bridge_token,};
use crate::utils::is_bridge_token;
use crate::state::BRIDGE_TOKEN;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};
use super::mock::ETH_ADDRESS;

// test everything from internal never use the message
#[test]
fn test_add_bridge_token() {
    let eth_address = ETH_ADDRESS;
    let cw20_address = Addr::unchecked("bridge token");

    let mut deps = mock_dependencies();
    // let mut storage = mock_storage()

    // BRIDGE_TOKEN.load(&deps.storage, &cw20_address)

    assert!(!is_bridge_token(&deps.storage, &cw20_address));

    add_bridge_token(&mut deps.storage, &eth_address, &cw20_address).unwrap();

    assert!(is_bridge_token(&deps.storage, &cw20_address));

    let cw20_in_state = get_cw20_via_eth_address(&deps.storage, &eth_address);

    assert_eq!(cw20_in_state, Some(cw20_address));
}
