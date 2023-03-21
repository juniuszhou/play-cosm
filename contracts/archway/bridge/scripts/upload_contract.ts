import { Contract, getMnemonic } from "./helpers/utils";
import { connect } from "./helpers/connect";
import { Constantine } from "./networks";
import { hitFaucet } from "./helpers/hitFaucet";
import { uploadContracts } from "./helpers/uploadContracts";
import { initToken } from "./helpers/initToken";

const contracts: Contract[] = [
  {
    name: "cw20_base",
    wasmFile: "../counter/artifacts/counter.wasm",
  },
];

async function main(): Promise<void> {
  const config = Constantine
  const mnemonic = getMnemonic();
  const {client, address} = await connect(mnemonic, config)
  let {amount} = await client.getBalance(address, config.feeToken)
  console.log("current amount is ", amount);

  if (amount === "0") {
    await hitFaucet(address, config.feeToken, config.faucetUrl)
    let {amount} = await client.getBalance(address, config.feeToken)
    console.log(`new balance is ${amount}`)
  }

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
