#

## docs
https://docs.ignite.com/guide/

## install 
curl https://get.ignite.com/cli! | bash

## current version
bash-5.0$ ignite version
Ignite CLI version:     v0.25.1

## start the chain 
ignite chain serve

## commands
`ignite scaffold module`

`ignite scaffold message`

ignite scaffold message postBlog title:string id:int


`ignite scaffold list` create a list type of objects


## add a new ibc packet 
`ignite scaffold packet ibcPost title content --ack postID` for ibc packet definition

