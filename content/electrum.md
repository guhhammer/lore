# electrum

Ergonomic Bitcoin wallet and private key management.

## Usage
- `electrum -w path/to/new_wallet.dat create` : create a new wallet
- `electrum -w path/to/recovery_wallet.dat restore -o` : restore an existing wallet from seed offline
- `electrum mktx recipient amount -f 0.0000001 -F from -o` : create a signed transaction offline
- `electrum listaddresses -a` : display all wallet receiving addresses
- `electrum signmessage address message` : sign a message
- `electrum verifymessage address signature message` : verify a message
- `electrum -p socks5:127.0.0.1:9050 -s 56ckl5obj37gypcu.onion:50001:t -1` : connect only to a specific electrum-server instance

## Links
- More information: https://manned.org/electrum
- tldr page: https://tldr.inbrowser.app/pages/common/electrum
