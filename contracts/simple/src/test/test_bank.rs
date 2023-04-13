use crate::test::COSMOS;

#[cfg(test)]
mod tests {
    // use crate::message::*;
    use cosmwasm_std::{Addr, BankMsg, Coin, Uint128, coins};
    use cw_multi_test::{App, Executor};

    use super::*;

    #[test]
    fn bank_transfer() {
        let sender = Addr::unchecked("sender");
        let receiver = Addr::unchecked("receiver");
        let init_amount = 1000;

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(init_amount, COSMOS))
                .unwrap();
            // storage.set()
        });

        let message = BankMsg::Send {
            to_address: receiver.to_string(),
            amount: vec![Coin{
                denom: COSMOS.to_string(),
                amount: Uint128::from(100_u32),
            }],
        };

        let result = app.execute(sender, message.into());
        assert_eq!(result.unwrap().data, None);
    }
}