import { SigningCosmWasmClient } from "cosmwasm";
import { Cw20Coin, InstantiateMsg } from "../bindings/Cw20.types";
import { Cw20Client, Cw20QueryClient } from "../bindings/Cw20.client";

export async function initToken(
  client: SigningCosmWasmClient,
  address: string,
  code: number
) {
  const initial_balances: Cw20Coin[] = [{ address, amount: "123456000000" }];
  const initMsg: InstantiateMsg = {
    name: "Test Token",
    symbol: "TTOKEN",
    decimals: 6,
    initial_balances,
    mint: {
      minter: address,
    },
  };

  const info = await client.instantiate(
    address,
    code,
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    initMsg,
    "Test Token 1.0",
    "auto",
    {
      admin: address,
    }
  );
  return info.contractAddress;
}


export async function getTokenInfo(
  client: SigningCosmWasmClient,
  address: string,
) {

  const cw20Client = new Cw20QueryClient(client, address);
  let cw20Address = cw20Client.contractAddress;
  console.log('address is ', cw20Address);

  let result = await cw20Client.tokenInfo();
  console.log("toke info ", result);
  
}