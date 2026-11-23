# sui client

Publish smart contracts, get object information, execute transactions, and more.

## Usage
- `sui client new-address ed25519 address_alias` : create a new address with the ED25519 scheme
- `sui client new-env --rpc https://fullnode.testnet.sui.io:443 --alias testnet` : create a new testnet environment with an RPC URL and alias
- `sui client switch --address address_alias` : switch to the address of your choice (accepts also an alias)
- `sui client switch --env env_alias` : switch to the given environment
- `sui client publish path/to/package` : publish a smart contract
- `sui client faucet subcommand` : interact with the Sui faucet
- `sui client gas address` : list the gas coins for the given address (accepts also an alias)
- `sui client ptb options subcommand` : create, sign, and execute programmable transaction blocks

## Categories
network, shell

## Links
- More information: https://docs.sui.io/references/cli/client
- tldr page: https://tldr.inbrowser.app/pages/common/sui-client
