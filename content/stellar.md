# stellar

Build, deploy, and interact with Stellar Soroban smart contracts and networks.

## Usage
- `stellar contract build` : build a Soroban smart contract in the current workspace
- `stellar keys generate identity_name -n testnet --fund` : generate a new keypair identity and fund it on the Stellar testnet
- `stellar contract deploy --wasm path/to/contract.wasm -s identity_name -n testnet|mainnet` : deploy a compiled smart contract to a network
- `stellar contract invoke --id contract_id -s identity_name -n testnet|mainnet -- function_name` : invoke a function on a deployed smart contract
- `stellar network ls` : list configured Stellar networks
- `stellar -V` : display version

## Categories
development, network

## Links
- More information: https://developers.stellar.org/docs/tools/cli/stellar-cli
- tldr page: https://tldr.inbrowser.app/pages/common/stellar
