import {
  DirectSecp256k1HdWallet,
  makeCosmoshubPath,
  SigningCosmWasmClient,
  CosmWasmClient,
} from "cosmwasm";

import { Network } from "../networks";
import {getMnemonic} from "./utils";

const AccountIndex: number = 0;

export async function getAccountAddress(network: Network) {
  const mnemonic = getMnemonic();
  const { prefix, gasPrice, rpcEndpoint } = network;
  const hdPath = makeCosmoshubPath(0);
  
  // Setup signer
  const offlineSigner = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix,
    hdPaths: [hdPath],
  });

  console.log(`Connected to ${mnemonic}`);

  const { address } = (await offlineSigner.getAccounts())[AccountIndex];
  

  return address;
}

export async function getSigningCosmWasmClient(network: Network) {
  const mnemonic = getMnemonic();
  const { prefix, gasPrice, rpcEndpoint } = network;
  const hdPath = makeCosmoshubPath(0);
  const AccountIndex = 0;

  // Setup signer
  const offlineSigner = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix,
    hdPaths: [hdPath],
  });

  console.log(`Connected to ${mnemonic}`);

  const { address } = (await offlineSigner.getAccounts())[0];
  console.log(`Connected to ${address}`);

  // Init SigningCosmWasmClient client
  const client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    offlineSigner,
    {
      prefix,
      gasPrice,
    }
  );

  return { client, address };
}

// cosmwasm client for query
export async function getCosmWasmClient(network: Network) {
  
  const client = CosmWasmClient.connect(
    network.rpcEndpoint,
  );

  return client;
}