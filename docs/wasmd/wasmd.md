# wasmd document

## wasmd repo https://github.com/juniuszhou/wasmd
branch dev

## commands

### compilation

```
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
wasm-debug = "build --target wasm32-unknown-unknown"
```

RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release


### upload contract to chain 
```
wasmd tx wasm upload target/wasm32-unknown-unknown/release/simple.wasm --from alice --chain-id wasmd --gas 10000000 -y
```



### instantiate
```
wasmd tx wasm instantiate 4 \
'{ }' \
--from alice --chain-id wasmd --gas 10000000 --label "label" --no-admin -y
```


### query contract
wasmd q wasm contract-state smart wasm1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq8epk7w


### query data, input as enum
$ wasmd query wasm contract-state smart \
  wasm1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrq8epk7w \
   '{"Greet":{}}'  --chain-id wasmd 


### send tx to contract


$ wasmd tx wasm upload target/ \
  '{ "admin": "wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux", "members": [] }' \
  --from wallet --label "Group" --no-admin $TXFLAG -y


