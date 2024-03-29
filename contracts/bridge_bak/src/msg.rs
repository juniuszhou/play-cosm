use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Addr};
use cw20::{Balance, Cw20CoinVerified};
use cw_controllers::AdminResponse;

// For bridge contract, we need admin to do lot of things.
// So use the sender as default admin, can update it via admin interface
#[cw_serde]
pub struct InstantiateMsg {
    pub cw20_code_id: u64,
}

#[cw_serde]
pub enum CosmosToken {
    Native(Coin),
    CW20(Cw20CoinVerified),
}

#[cw_serde]
pub struct EthClaim {
    // hash of ( sender address + sender nonce )
    pub claim_hash: String,
    // 0x00000000000000000 for eth
    pub token_address: String,
    // None is for lock ethereum token
    // Some for burn token back to cosmos
    pub cosmos_token: Option<CosmosToken>,
    // one token could distribute to different receivers
    pub receiver: String,
    pub amount: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateAdmin {
        admin: String,
    },
    Lock {
        balances: Vec<Balance>,
        receiver: String,
    },
    Burn {
        balances: Vec<Cw20CoinVerified>,
        receiver: String,
    },
    BridgeClaim {
        claims: Vec<EthClaim>,
    },
    ExecuteTest {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AdminResponse)]
    QueryAdmin {},
    #[returns(Cw20AddressResponse)]
    QueryCw20Address {erc20_address: String},
}

#[cw_serde]
pub struct QueryAdmin {}

#[cw_serde]
pub struct QueryCw20Address {
    pub erc20_address: String,
}

#[cw_serde]
pub struct Cw20AddressResponse {
    pub cw20_address: Option<Addr>,
}

#[cw_serde]
pub struct UpdateAdmin {
    pub admin: String,
}
