pub mod contract;
pub mod error;
pub mod msg;
mod response;
mod state;
mod utils;

#[cfg(test)]
mod tests;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const NATIVE_TOKEN_LOCK: &str = "native_token_lock";
const CW20_TOKEN_LOCK: &str = "cw20_token_lock";
const CW20_TOKEN_BURN: &str = "cw20_token_burn";
