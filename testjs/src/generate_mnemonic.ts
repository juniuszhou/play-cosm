import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"

let prefix = "wasm"
const generateKey = async (): Promise<void> => {
    // const option = DirectSecp256k1HdWalletOptions
    const wallet: DirectSecp256k1HdWallet = await DirectSecp256k1HdWallet.generate(24, {prefix: prefix})
    process.stdout.write(wallet.mnemonic)
    // wallet.
    const accounts = await wallet.getAccounts()
    console.error("Mnemonic with 1st account:", accounts[0].address)
}

generateKey()

