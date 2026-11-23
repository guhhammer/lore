# bitcoin-cli

Client to interact with the Bitcoin Core daemon via RPC calls. Uses the configuration defined in `bitcoin.conf`.

## Usage
- `bitcoin-cli sendtoaddress "address" amount` : send a transaction to a given address
- `bitcoin-cli generate num_blocks` : generate one or more blocks
- `bitcoin-cli getwalletinfo` : print high-level information about the wallet
- `bitcoin-cli listunspent` : list all outputs from previous transactions available to fund outgoing transactions
- `bitcoin-cli dumpwallet "path/to/file"` : export the wallet information to a text file
- `bitcoin-cli getblockchaininfo` : get blockchain information
- `bitcoin-cli getnetworkinfo` : get network information
- `bitcoin-cli stop` : stop the Bitcoin Core daemon

## Categories
system, files, network

## Links
- More information: https://en.bitcoin.it/wiki/Running_Bitcoin#Command-line_arguments
- tldr page: https://tldr.inbrowser.app/pages/common/bitcoin-cli
