// data from https://github.com/cosmos/chain-registry/tree/master/testnets
import { GasPrice } from "@cosmjs/stargate";

export interface Network {
  chainId: string;
  rpcEndpoint: string;
  prefix: string;
  gasPrice: GasPrice;
  feeToken: string;
  faucetUrl: string;
}

export const malagaConfig: Network = {
  chainId: "malaga-420",
  rpcEndpoint: "https://rpc.malaga-420.cosmwasm.com/",
  prefix: "wasm",
  gasPrice: GasPrice.fromString("0.25umlg"),
  feeToken: "umlg",
  faucetUrl: "https://faucet.malaga-420.cosmwasm.com/",
};

export const localConfig: Network = {
  chainId: "testing",
  rpcEndpoint: "http://localhost:26657",
  prefix: "wasm",
  gasPrice: GasPrice.fromString("0.25ucosm"),
  feeToken: "ucosm",
  faucetUrl: "https://faucet.malaga-420.cosmwasm.com/",
};


export const Torii: Network = {
  chainId: "torii-1",
  rpcEndpoint: "https://rpc.torii-1.archway.tech",
  prefix: "archway",
  gasPrice: GasPrice.fromString("0.25uarch"),
  feeToken: "uarch",
  faucetUrl: "https://faucet.torii-1.archway.tech",
};



export const Constantine: Network = {
  chainId: "constantine-1",
  rpcEndpoint: "https://rpc.constantine-1.archway.tech",
  prefix: "archway",
  gasPrice: GasPrice.fromString("0.25uconst"),
  feeToken: "uconst",
  faucetUrl: "https://faucet.constantine-1.archway.tech",
};