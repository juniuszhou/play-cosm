use serde::{Deserialize, Serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    // pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Execute {},
    UpdateAdmin {admin: String},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(GreetResp)]
    QueryAdmin {},
    #[returns(GreetResp)]
    QueryMyMap { key: u64},
    #[returns(GreetResp)]
    QueryMyItem {},
}

#[cw_serde]
pub struct Greet {
    pub message: String,
}

#[cw_serde]
pub struct GreetResp {
    pub message: String,
}

#[cw_serde]
pub struct QueryAdmin {
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
pub struct QueryMyItem {
}

#[cw_serde]
pub struct QueryMyItemResp {
    pub result: String,
}

#[cw_serde]
pub struct UpdateAdmin {
    pub admin: String,
}