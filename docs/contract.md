# cosmwasm 

## book 
https://book.cosmwasm.com/

## command 

### query state
`wasmd query wasm contract-state `







### code could be pinned in memory
Code Pinning mechanism allows codes to be pinned to the memory.

This way code does not have to be loaded to memory on each execution thus makes ~x40 performance.

This is an estimation, needs to be benchmarked.

Code pinning is done through native chain governance.