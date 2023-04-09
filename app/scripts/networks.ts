// data from https://github.com/cosmos/chain-registry/tree/master/testnets
import { GasPrice } from "@cosmjs/stargate";
import { readFileSync } from "fs";

export interface Network {
  chainId: string;
  rpcEndpoint: string;
  prefix: string;
  gasPrice: GasPrice;
  feeToken: string;
  faucetUrl: string;
}

export const Constantine: Network = {
  chainId: "constantine-1",
  rpcEndpoint: "https://rpc.constantine-1.archway.tech",
  prefix: "archway",
  gasPrice: GasPrice.fromString("0.25uconst"),
  feeToken: "uconst",
  faucetUrl: "https://faucet.constantine-1.archway.tech",
};


export const Constantine2: Network = {
  chainId: "constantine-2",
  rpcEndpoint: "https://rpc.constantine-2.archway.tech",
  prefix: "archway",
  gasPrice: GasPrice.fromString("0.25uconst"),
  feeToken: "uconst",
  faucetUrl: "https://faucet.constantine-2.archway.tech",
};

function read_config() {
  let file = readFileSync('config.json');
  const content = file.toString();
  const json = JSON.parse(content);
  // console.log('network is ', json['network'])

  const network =  {
    chainId: json['network'],
    rpcEndpoint: json['rpcEndpoint'],
    prefix: json['prefix'],
    GasPrice: json['gasPrice'],
    feeToken: json['feeToken'],
    faucetUrl: json['faucetUrl'],
  };

  // console.log('network is ', network);

  return network;
}

// read_config();
