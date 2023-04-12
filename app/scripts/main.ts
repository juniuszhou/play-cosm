
import { Constantine, Constantine2 } from "./networks";
import {getCw20QueryClient, getCW20SigningClient, transfer} from "./helpers/arch";
import { instantiateContracts } from "./helpers/uploadContracts";
import {getSigningCosmWasmClient} from "./helpers/connect";

async function main(): Promise<void> {
    const network = Constantine2;
    const recipient = "archway16wpz72yrqyvgpqs2z64fl29le3javgujr86g2zxn2djwuuxcakusd5tt8m";

    const cw20Address = "archway16hhhhh3g4g8klxk35nghj2ffmjafnf8cane3462hujlk72plp9nqvl5nr8";

    const codeId = 466;

    // const client = await getCw20QueryClient(network, cw20Address);
    // console.log('token info is ', await client.tokenInfo());

    const { client, address} = await getSigningCosmWasmClient(network);



    // console.log('address is ', await cw20SigningClient.balance({address}));

    // console.log('minter is ', await cw20SigningClient.minter());

    
    // await transfer(cw20SigningClient, "1", recipient);

    // await cw20SigningClient.mint({amount: "123456", recipient: address});

    // console.log('address is ', await cw20SigningClient.balance({address}));

    await instantiateContracts(client, address, codeId);

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
