import { SigningCosmWasmClient, InstantiateResult } from "cosmwasm";
import { Contract, loadContract } from "./utils";

import { InstantiateMsg } from "../bindings/Cw20.types";

interface UploadResults {
  [name: string]: number;
}

/**
 * 
 * @param client 
 * @param signer 
 * @param contracts 
 * @returns 
 */
export async function uploadContracts(
  client: SigningCosmWasmClient,
  signer: string,
  contracts: Contract[]
): Promise<UploadResults> {
  const uploaded: UploadResults = {};
  for (const contract of contracts) {
    const wasm = await loadContract(contract);
    console.debug(`Uploading ${contract.name}...`);
    const receipt = await client.upload(
      signer,
      wasm,
      "auto",
      `Upload ${contract.name}`
    );
    uploaded[contract.name] = receipt.codeId;
  }
  return uploaded;
}

export async function instantiateContracts(
  client: SigningCosmWasmClient,
  signer: string,
  codeId: number,
  // msg: InstantiateMsg
): Promise<InstantiateResult> {

  // const m: InstantiateMsg = {
  //   decimals: 18,
  //   initial_balances: [],
  //   marketing: null,
  //   mint: null,
  //   name: 'name',
  //   symbol: 'name'
  // };

  const result = await client.instantiate(signer, codeId, {
    decimals: 18,
    initial_balances: [],
    marketing: null,
    mint: null,
    name: 'name',
    symbol: 'name'
  }, "auto", "auto", { admin: signer });

  console.log("result is: ", result.contractAddress);

  return result;
}
