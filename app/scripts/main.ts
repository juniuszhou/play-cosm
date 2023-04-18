
import { Constantine, Constantine2 } from "./networks";
import { getMnemonic } from "./helpers/utils";
import {getSigningCosmWasmClient, getCosmWasmClient, getAccountAddress} from "./helpers/connect";

async function main(): Promise<void> {
    const network = Constantine2;
    const client = await getCosmWasmClient(network);
    const address = await getAccountAddress(network)
    const account = await client.getAccount(address);
    console.log("account is ", account);

    const balance = await client.getBalance(address, network.feeToken);

    console.log("balance is ", balance);

}

main().then(
  () => {
    process.exit(0);
  },
  (error) => {
    console.error(error);
    process.exit(1);
  }
);
