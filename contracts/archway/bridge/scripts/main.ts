import { Contract, getMnemonic } from "./helpers/utils";
import { connect } from "./helpers/connect";
import { Constantine } from "./networks";
import { hitFaucet } from "./helpers/hitFaucet";
import { uploadContracts } from "./helpers/uploadContracts";
import { initToken, getTokenInfo } from "./helpers/initToken";

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

  const cw20Address = "archway1tl7af6tcznnss7qzmcncuex2apdtcp0uwjwqq33ragpc4vpk2s9q74g6cm";
  await getTokenInfo(client, cw20Address);

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
