use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_controllers::AdminResponse;
use cosmwasm_std::{Coin, Addr};

// For bridge contract, we need admin to do lot of things.
// So use the sender as default admin, can update it via admin interface
#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    Execute {},
    UpdateAdmin { admin: String },
    // Lock {
    //     coin: Coin,
    //     receiver: String
    // },
    // Burn {
    //     address: Addr,
    //     amount: u128,
    // }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(AdminResponse)]
    QueryAdmin {},
}

#[cw_serde]
pub struct QueryAdmin {}

#[cw_serde]
pub struct GreetResp {
    pub message: String,
}

#[cw_serde]
pub struct QueryMyMap {
    pub key: u64,
}

#[cw_serde]
pub struct QueryMyMapResp {
    pub result: String,
}

#[cw_serde]
pub struct QueryMyItem {}

#[cw_serde]
pub struct QueryMyItemResp {
    pub result: String,
}

#[cw_serde]
pub struct UpdateAdmin {
    pub admin: String,
}
