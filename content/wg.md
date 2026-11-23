# wg

Manage the configuration of WireGuard interfaces.

## Usage
- `sudo wg` : check status of currently active interfaces
- `sudo wg show interface` : check status of the specified interface
- `wg genkey` : generate a new private key and print it to `stdout`
- `wg <path/to/private_key pubkey` : generate a public key from a private key and print it to `stdout`
- `wg genkey | tee path/to/private_key | wg pubkey >path/to/public_key` : generate a public and private key and save them to files
- `wg genpsk` : generate a pre-shared key and print it to `stdout`
- `sudo wg showconf interface` : show the active configuration of an interface

## Categories
network, documents, files

## Links
- More information: https://www.wireguard.com/quickstart/
- tldr page: https://tldr.inbrowser.app/pages/common/wg
