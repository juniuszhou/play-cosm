
import { IndexedTx, StargateClient } from "@cosmjs/stargate"
import { MsgSend } from "cosmjs-types/cosmos/bank/v1beta1/tx"
import { Tx } from "cosmjs-types/cosmos/tx/v1beta1/tx"

// const rpc = "rpc.sentry-01.theta-testnet.polypore.xyz:26657"

const rpc = "http://0.0.0.0:26657"
// address for alice
const address = "wasm1wslklgldzdzmqkx95se2m3sj5hnecgrk4szf6r"

// use the client to get all basic info
async function printAll() {
  const client = await StargateClient.connect(rpc)
  console.log("chain id:", await client.getChainId())
  console.log("chain height:", await client.getHeight())
  console.log("get account info:", await client.getAccount(address))
  /*
  chain height: {
  address: 'wasm1wslklgldzdzmqkx95se2m3sj5hnecgrk4szf6r',
  pubkey: {
    type: 'tendermint/PubKeySecp256k1',
    value: 'AuGIXROBYS2uGB62FelVMfVETOrLpo39ssmomdGpvv4t'
  },
  accountNumber: 0,
  sequence: 13
}
  */


  // get sequence 
  console.log("address sequence:", await (await client.getSequence(address)).sequence)
  // address sequence: { accountNumber: 0, sequence: 13 }

  console.log("account balance:", await client.getAllBalances(address))
  /*
  account balance: [
  { denom: 'stake', amount: '100000000' },
  { denom: 'token', amount: '20000' }
  ]
  */
}

printAll()