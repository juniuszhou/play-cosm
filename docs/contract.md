# cosmwasm 

## book 
https://book.cosmwasm.com/

## command 

### query state
`wasmd query wasm contract-state `





### feature list
1. call other contract
2. trigger the method defined in bank module
3. trigger the custom method like post blog in blog chain
4. trigger the ibc communication 
5. type script to test against wasm 
6. type script to test against ibc
7. gas computing and used.


### code could be pinned in memory
Code Pinning mechanism allows codes to be pinned to the memory.

This way code does not have to be loaded to memory on each execution thus makes ~x40 performance.

This is an estimation, needs to be benchmarked.

Code pinning is done through native chain governance.


## relayer management 
admin account
ibc message trusted 
denom mapping from outside to internal 

from eth to cosmos 
validators and threshold , power 

denom, from chain_id, ethereum address, map to ics20 contract address. 

user can burn the peggy contract 
use can lock issued token 
