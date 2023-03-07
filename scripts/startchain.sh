#!/bin/sh
#set -o errexit -o nounset -o pipefail

PASSWORD=${PASSWORD:-1234567890}
STAKE=${STAKE_TOKEN:-stake}
FEE=${FEE_TOKEN:-ucosm}
CHAIN_ID=${CHAIN_ID:-testing}
MONIKER=${MONIKER:-node001}
BINARY=${BINARY:-wasmd}

rm -rf "$HOME"/".$BINARY"/config
rm -rf "$HOME"/".$BINARY"/data
rm -rf "$HOME"/".$BINARY"/wasm

$BINARY init --chain-id "$CHAIN_ID" "$MONIKER"
# staking/governance token is hardcoded in config, change this
sed -i "s/\"stake\"/\"$STAKE\"/" "$HOME"/".$BINARY"/config/genesis.json
# this is essential for sub-1s block times (or header times go crazy)
sed -i 's/"time_iota_ms": "1000"/"time_iota_ms": "10"/' "$HOME"/".$BINARY"/config/genesis.json

if ! $BINARY keys show validator; then
  (echo "$PASSWORD"; echo "$PASSWORD") | $BINARY keys add validator
fi
# hardcode the validator account for this instance
echo "$PASSWORD" | $BINARY add-genesis-account validator "1000000000$STAKE,1000000000$FEE"

# (optionally) add a few more genesis accounts
for addr in "$@"; do
  echo $addr
  $BINARY add-genesis-account "$addr" "1000000000$STAKE,1000000000$FEE"
done

$BINARY add-genesis-account wasm1uewj8cle2tps32yq5gk5lzqf0zgu8vmxwpadkw "1000000000$STAKE,1000000000$FEE"

# submit a genesis validator tx
## Workraround for https://github.com/cosmos/cosmos-sdk/issues/8251
(echo "$PASSWORD"; echo "$PASSWORD"; echo "$PASSWORD") | $BINARY gentx validator "250000000$STAKE" --chain-id="$CHAIN_ID" --amount="250000000$STAKE"
## should be:
# (echo "$PASSWORD"; echo "$PASSWORD"; echo "$PASSWORD") | $BINARY gentx validator "250000000$STAKE" --chain-id="$CHAIN_ID"
$BINARY collect-gentxs


$BINARY start --rpc.laddr tcp://0.0.0.0:26657 --trace