import { Contract, getMnemonic } from "./helpers/utils";
import { connect } from "./helpers/connect";
import { Constantine } from "./networks";
import { hitFaucet } from "./helpers/hitFaucet";
import { uploadContracts } from "./helpers/uploadContracts";
import { initToken, getTokenInfo, signTx } from "./helpers/initToken";

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

  console.log("account address is ", address);
  let {amount} = await client.getBalance(address, config.feeToken)
  console.log("current amount is ", amount);

  const cw20Address = "archway133j5datpdfh6td26w6xsvpnj335csnn0v02u4lvy5hfucs3j3euqxult48";
  await getTokenInfo(client, cw20Address);

  await signTx(client, address, cw20Address);

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
