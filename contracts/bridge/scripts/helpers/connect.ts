import {
  DirectSecp256k1HdWallet,
  makeCosmoshubPath,
  SigningCosmWasmClient,
} from "cosmwasm";

import { Network } from "../networks";
const AccountIndex = 1;

export async function getSigningCosmWasmClient(mnemonic: string, network: Network) {
  const { prefix, gasPrice, rpcEndpoint } = network;
  const hdPath = makeCosmoshubPath(0);

  // Setup signer
  const offlineSigner = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix,
    hdPaths: [hdPath],
  });
  const { address } = (await offlineSigner.getAccounts())[AccountIndex];
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
  // const balance = await client.getBalance(address, feeToken);
  // console.log(`Balance: ${balance.amount} ${balance.denom}`);

  // const chainId = await client.getChainId();

  // if (chainId !== network.chainId) {
  //   throw Error("Given ChainId doesn't match the clients ChainID!");
  // }

  return { client, address };
}
