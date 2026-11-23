# octez-client

Interact with the Tezos blockchain.

## Usage
- `octez-client -E endpoint config update` : configure the client with a connection to a Tezos RPC node such as <https://rpc.ghostnet.teztnets.com>
- `octez-client gen keys alias` : create an account and assign a local alias to it
- `octez-client get balance for alias_or_address` : get the balance of an account by alias or address
- `octez-client transfer 5 from alias|address to alias|address` : transfer tez to a different account
- `octez-client originate contract alias transferring 0 from alias|address running path/to/source_file.tz --init "initial_storage" --burn_cap 1` : originate (deploy) a smart contract, assign it a local alias, and set its initial storage as a Michelson-encoded value
- `octez-client transfer 0 from alias|address to contract --entrypoint "entrypoint" --arg "parameter" --burn-cap 1` : call a smart contract by its alias or address and pass a Michelson-encoded parameter
- `octez-client man` : display help

## Categories
shell, network, users

## Links
- More information: https://tezos.gitlab.io/introduction/howtouse.html#client
- tldr page: https://tldr.inbrowser.app/pages/common/octez-client
