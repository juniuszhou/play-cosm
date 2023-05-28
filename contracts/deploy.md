# contract deploy in archway

archway deploy 
 archway deploy --no-build


## counter

Chain Id: constantine-1
  Tx Hash:  CEE8CFC4B3D4077939089200C24443A0AC6A67C0BAD5BE501D16EE573A52DCDC
  Code Id:  223

Successfully instantiated the contract
  Chain Id:  constantine-1
  Tx Hash:   06A3272767694C700DCD9FCC74F055581BE65B1E1C34F2D9F7F50401CD46D733
  Address:   archway1uacxg46phsdtqrdnc45wnyqp9vl6rgvr03ms5mk6vmqz9jl9m6gsa2d26r
  Arguments: {"count": 0}


## new cw 20 code
✔ Waiting for tx D68D4C5AED9265B4C849CF7FE0916713276E144F37EDD6127ED7C173BC542261 to confirm...

File /Users/junius/github/junius/play-cosm/contracts/archway/cw20/artifacts/cw20.wasm successfully uploaded
  Chain Id: constantine-1
  Tx Hash:  D68D4C5AED9265B4C849CF7FE0916713276E144F37EDD6127ED7C173BC542261
  Code Id:  298

✔ Downloading wasm file from constantine-1...
Integrity check Ok!

Instantiating cw20 0.1.0 from code id 298 on constantine-1...
Fetching address for wallet j...
archway1yhjvnzwcp0mgzcpzsu8nd7yz2wzy9jc9sa44dw
✔ Waiting for tx 4E1DB1FF20E4955851322C8B434926C649CE7B6092838244F9A285D965280829 to confirm...

Successfully instantiated the contract
  Chain Id:  constantine-1
  Tx Hash:   4E1DB1FF20E4955851322C8B434926C649CE7B6092838244F9A285D965280829
  Address:   archway133j5datpdfh6td26w6xsvpnj335csnn0v02u4lvy5hfucs3j3euqxult48
  Arguments:   
  
  
  ## arguments
  {"name": "name", "symbol": "symbol", "decimals": 18, "initial_balances": [], "minter_response": {"minter": "archway1yhjvnzwcp0mgzcpzsu8nd7yz2wzy9jc9sa44dw", "cap": ""}}

 {"name": "name", "symbol": "symbol", "decimals": 18, "initial_balances": [{"archway1g6tzv6qt5d358ylmlmyhwxe09uhq39dcfvs2fs": "100000000000000"}]}
 
 
 
 
 "minter_response": {"minter": "archway1yhjvnzwcp0mgzcpzsu8nd7yz2wzy9jc9sa44dw", "cap": ""}}


Optimized wasm binary saved to artifacts/cw20.wasm

✔ Send tx from which wallet in your keychain? (e.g. "main" or crtl+c to quit) … j
✔ Choose a label for this deployment (type <enter> to use the default) … cw20 0.1.0
✔ JSON encoded arguments for contract initialization (e.g. { "count": 0 }) … {"name": "name", "symbol": "symbol", "decimals": 18, "initial_balances": [], "minter_response": {"minter": "archway1yhjvnzwcp0mgzcpzsu8nd7yz2wzy9jc9sa44dw", "cap": ""}}
Uploading optimized wasm to constantine-2 using wallet j...
✔ Waiting for tx A3479F1AFD10470C1FCBF566F29C50039DA5805CFF8D22065DB8317B3F1D40AC to confirm...

File /Users/junius/github/junius/play-cosm/contracts/archway/cw20/artifacts/cw20.wasm successfully uploaded
  Chain Id: constantine-2
  Tx Hash:  A3479F1AFD10470C1FCBF566F29C50039DA5805CFF8D22065DB8317B3F1D40AC
  Code Id:  461

✔ Downloading wasm file from constantine-2...
Integrity check Ok!

Instantiating cw20 0.1.0 from code id 461 on constantine-2...
Fetching address for wallet j...
archway1g6tzv6qt5d358ylmlmyhwxe09uhq39dcfvs2fs
✔ Waiting for tx D117D7BB2742856713864C035F891BEA82226A9B756FF90E38E83547652481C4 to confirm...

Successfully instantiated the contract
  Chain Id:  constantine-2
  Tx Hash:   D117D7BB2742856713864C035F891BEA82226A9B756FF90E38E83547652481C4
  Address:   archway1j5gm590e3dwytxtl83lxx79594xdq5pkdussy6hahzllrz7p3ejs0umnxy
  Arguments: 
  
{"name": "name", "symbol": "symbol", "decimals": 18, "initial_balances": [], "minter_response": {"minter": "archway1g6tzv6qt5d358ylmlmyhwxe09uhq39dcfvs2fs", "cap": ""}}

## correct arguments for cw20
{"name": "name", "symbol": "symbol", "decimals": 18, "initial_balances": [{"address": "archway1g6tzv6qt5d358ylmlmyhwxe09uhq39dcfvs2fs", "amount": "100000000000000000"}]}


