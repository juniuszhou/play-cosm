import { Contract, getMnemonic } from "./helpers/utils";
import { connect } from "./helpers/connect";
import { malagaConfig } from "./networks";
import { hitFaucet } from "./helpers/hitFaucet";
import { uploadContracts } from "./helpers/uploadContracts";
import { initToken } from "./helpers/initToken";

const contracts: Contract[] = [
  {
    name: "cw20_base",
    wasmFile: "./contracts/cw20_base.wasm",
  },
];

async function main(): Promise<void> {
  /**
   *  We're going to upload & initialise the contract here!
   *  Check out the video course on academy.cosmwasm.com!
   */

  const mnemonic = getMnemonic();
  const {client, address} = await connect(mnemonic, malagaConfig)
  let {amount} = await client.getBalance(address, malagaConfig.feeToken)
  if (amount === "0") {
    await hitFaucet(address, malagaConfig.feeToken, malagaConfig.faucetUrl)
    let {amount} = await client.getBalance(address, malagaConfig.feeToken)
    console.log(`new balance is ${amount}`)
  }

  let {codeId} = await uploadContracts(client, address, contracts)
  if (codeId === undefined) {
    console.log("upload contract failed, exit")
    return
  }
  console.log(`code is ${codeId}`)


  const contractAddress = await initToken(client, address, codeId)
  console.log(`contract address is ${contractAddress}`)
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
