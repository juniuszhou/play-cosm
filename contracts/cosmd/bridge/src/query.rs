use super::*;
use crate::msg::GreetResp;

pub fn greet() -> StdResult<GreetResp> {
    let resp = GreetResp {
        message: "Hello World".to_owned(),
    };

    Ok(resp)
}

