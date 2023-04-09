
import { Constantine } from "./networks";


import {getCw20QueryClient, getCW20SigningClient, transfer} from "./helpers/arch";


async function main(): Promise<void> {
    const network = Constantine;
    const recipient = "";
    const cw20Address = "archway133j5datpdfh6td26w6xsvpnj335csnn0v02u4lvy5hfucs3j3euqxult48";

    const client = await getCw20QueryClient(network, cw20Address);
    console.log('token info is ', await client.tokenInfo());

    const cw20SigningClient = await getCW20SigningClient(network, cw20Address);
    
    await transfer(cw20SigningClient, "1", recipient);

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
