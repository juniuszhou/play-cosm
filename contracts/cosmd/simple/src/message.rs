use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admin: String,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    Execute {},
    UpdateAdmin {admin: String},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
    QueryAdmin {},
    QueryMyMap { key: u64},
    QueryMyItem {},
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryAdmin {
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryMyMap {
    pub key: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryMyMapResp {
    pub result: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryMyItem {
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryMyItemResp {
    pub result: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UpdateAdmin {
    pub admin: String,
}