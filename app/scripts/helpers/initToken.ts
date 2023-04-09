import { SigningCosmWasmClient } from "cosmwasm";
import { toUtf8 } from "@cosmjs/encoding";
import {
  MsgExecuteContract,
  MsgStoreCode,
} from "cosmjs-types/cosmwasm/wasm/v1/tx";

import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";

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
  address: string
) {
  const cw20Client = new Cw20QueryClient(client, address);
  let cw20Address = cw20Client.contractAddress;
  console.log("address is ", cw20Address);

  let result = await cw20Client.tokenInfo();
  console.log("toke info ", result);

  let minter = await cw20Client.minter();
  console.log("minter is: ", minter);
}

export async function signTx(
  client: SigningCosmWasmClient,
  account: string,
  contractAddress: string
) {
  const transferMsg = {
    transfer: {
      recipient: account,
      amount: 1000,
    },
  };

  const msgExecuteContract = MsgExecuteContract.fromPartial({
    sender: account,
    contract: contractAddress,
    msg: toUtf8(JSON.stringify(transferMsg)),
    funds: [],
  });

  const executeContractMsg: MsgExecuteContractEncodeObject = {
    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
    value: msgExecuteContract,
  };

  const memo = "Transfer tokens";
  const result = await client.signAndBroadcast(account, [executeContractMsg], "auto", memo);
  console.log("Result: ", result);


  // const cw20Client = new Cw20QueryClient(client, address);

  // let transferBytes = toUtf8('{"transfer":{"amount":"1","recipient":"juno1cx4nq77x3unvl2xsa9fmm9drxkexzkjnzwt2y7"}}');
  // const msgExecuteContract = MsgExecuteContract.fromPartial({
  // 	sender: address,
  // 	contract: address,
  // 	msg: transferBytes,
  // 	funds: []
  // });

  // const executeContractMsg: MsgExecuteContractEncodeObject = {
  //   typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
  //   value: msgExecuteContract
  // };
  // const memo = "Go go go";
  // const gasUsed = await client.simulate(alice.address0, [executeContractMsg], memo);

  // let result = await client.execute(account, address, { transfer: {recipient: address, amount: 1} }, "auto");
  // console.log('result is ' , result);
  // cw20Client.client.broadcastTx();
}
