import { Contract, getMnemonic } from "./helpers/utils";
import { Constantine, Constantine2 } from "./networks";
import { uploadContracts } from "./helpers/uploadContracts";
import {getSigningCosmWasmClient} from "./helpers/connect"

const contracts: Contract[] = [
  {
    name: "cw20",
    wasmFile: "./scripts/contracts/cw20.wasm",
  },
];

async function main(): Promise<void> {

  const config = Constantine2;

  console.log( "Constantine is ", Constantine);

  const mnemonic = getMnemonic();

  const {client, address} = await getSigningCosmWasmClient(mnemonic, config)

  let {amount} = await client.getBalance(address, config.feeToken)
  console.log("current amount is ", amount);

  let {codeId} = await uploadContracts(client, address, contracts)
  if (codeId === undefined) {
    console.log("upload contract failed, exit")
    return
  }
  console.log(`code is ${codeId}`)


  // const contractAddress = await initToken(client, address, codeId)
  // console.log(`contract address is ${contractAddress}`)
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
