use ethaddr::Address;

fn main() {
    let s = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
    let address = s.parse::<Address>().unwrap();

    println!("address is ${address}");
}
