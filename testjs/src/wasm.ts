import {CosmWasmClient, SigningCosmWasmClient, BroadcastMode} from "@cosmwasm/sdk"

const contract_root = "/Users/junius/github/junius/play-cosm/contracts"
const rpc = "http://0.0.0.0:1317"

// address for alice
const address = "wasm1wslklgldzdzmqkx95se2m3sj5hnecgrk4szf6r"

function deploy_contract() {
    // SigningCosmWasmClient for deploy and upload code

}

async function get_code() {
    const client = new CosmWasmClient(rpc, BroadcastMode.Async)

    const id = await client.getChainId()
    console.log(id)

    // get all codes from wasmd
    console.log(await client.getCodes())
}


get_code()

// get a cosmwasm client and deploy contract 