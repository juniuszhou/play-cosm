
import { IndexedTx, StargateClient } from "@cosmjs/stargate"
import { MsgSend } from "cosmjs-types/cosmos/bank/v1beta1/tx"
import { Tx } from "cosmjs-types/cosmos/tx/v1beta1/tx"
import {getAddressFromName} from "./keys"

// get the network type as second argument
let network = process.argv[2]

function getRpc() {
  if (network == "official") {
    return "rpc.sentry-01.theta-testnet.polypore.xyz:26657"
  } else {
    return "http://0.0.0.0:26657"
  }
}

function getAddress() {
  if (network == "official") {
    return 'cosmos17tvd4hcszq7lcxuwzrqkepuau9fye3dal606zf'
  } else if (network == "wasm") {
    // alice address
    return getAddressFromName("wasmd", "alice")
  } else {
    return getAddressFromName("blogd", "alice")
  }
}

let rpc = getRpc()
let address = getAddress()

// use the client to get all basic info
async function printAll() {
  const client = await StargateClient.connect(rpc)
  console.log("chain id:", await client.getChainId())
  console.log("chain height:", await client.getHeight())
  console.log("get account info:", await client.getAccount(address))

  // get sequence 
  console.log("address sequence:", await (await client.getSequence(address)).sequence)

  // address sequence: { accountNumber: 0, sequence: 13 }
  console.log("account balance:", await client.getAllBalances(address))

}

printAll()
