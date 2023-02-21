use ethaddr::{Address};
use cw20::InstantiateMsg;

fn main() {

    let s = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
    let address = s.parse::<Address>().unwrap();

    println!("address is ${address}");

}