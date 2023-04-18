import { SigningCosmWasmClient, InstantiateResult } from "cosmwasm";
import { Cw20Client } from "../bindings/Cw20.client";

const recipient = "archway16wpz72yrqyvgpqs2z64fl29le3javgujr86g2zxn2djwuuxcakusd5tt8m";

const cw20Address = "archway16hhhhh3g4g8klxk35nghj2ffmjafnf8cane3462hujlk72plp9nqvl5nr8";

const codeId = 466;

// instantiate a cw20 contract
export async function instantiateContracts(
  client: SigningCosmWasmClient,
  signer: string,
  codeId: number
): Promise<InstantiateResult> {
  const result = await client.instantiate(
    signer,
    codeId,
    {
      decimals: 18,
      initial_balances: [],
      marketing: null,
      mint: null,
      name: "name",
      symbol: "name",
    },
    "auto",
    "auto",
    { admin: signer }
  );

  console.log("instantiated contract address is: ", result.contractAddress);

  return result;
}


export async function transfer(cw20SigningClient: Cw20Client, amount: string, recipient: string) {
    const result = await cw20SigningClient.transfer({amount, recipient});
  
    console.log("result as ", result);
  }
