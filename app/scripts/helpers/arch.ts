import { ArchwayClient } from "@archwayhq/arch3.js";
import { SigningArchwayClient } from "@archwayhq/arch3.js";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { Constantine } from "../networks";
import { Contract, getMnemonic } from "../helpers/utils";
import { Cw20Client, Cw20QueryClient, } from "../bindings/Cw20.client";
import { Network } from "../networks";
import {getSigningCosmWasmClient} from "./connect";

export async function getArchClient(network: Network) {
  const client = await ArchwayClient.connect(
    network.rpcEndpoint
  );
  return client;
}

export async function getCw20QueryClient(network: Network, cw20Address: string) {
  const client = await getArchClient(network);

  const cw20Client = new Cw20QueryClient(client, cw20Address);

  return cw20Client;
}

export async function getCW20SigningClient(network: Network, cw20Address: string) {
  const mnemonic = getMnemonic();
  const { client, address } = await getSigningCosmWasmClient(mnemonic, network);
  const cw20SigningClient = new Cw20Client(client, address, cw20Address);
  return {cw20SigningClient, address};
}


export async function printCount() {
  const client = await ArchwayClient.connect(
    "https://rpc.constantine-1.archway.tech"
  );

  const contractAddress =
    "archway133j5datpdfh6td26w6xsvpnj335csnn0v02u4lvy5hfucs3j3euqxult48";
  const msg = {
    token_info: {},
  };

  const token_info = await client.queryContractSmart(contractAddress, msg);

  console.log("counter is ", token_info);
}

export async function transfer(cw20SigningClient: Cw20Client, amount: string, recipient: string) {
  const result = await cw20SigningClient.transfer({amount, recipient});

  console.log("result as ", result);
}
