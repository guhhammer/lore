# impacket-rpcmap

This command is an alias of `rpcmap.py`.

## Usage
- `rpcmap.py stringbinding` : connect to an MSRPC interface using a string binding (e.g., `ncacn_ip_tcp:host[port]`)
- `rpcmap.py -brute-uuids stringbinding` : bruteforce UUIDs even if the MGMT interface is available
- `rpcmap.py -brute-opnums stringbinding` : bruteforce operation numbers (opnums) for discovered UUIDs
- `rpcmap.py -brute-versions stringbinding` : bruteforce major versions of found UUIDs
- `rpcmap.py -target-ip ip_address stringbinding` : specify a target IP address manually
- `rpcmap.py -auth-rpc domain/username:password stringbinding` : authenticate to the RPC interface with username and password
- `rpcmap.py -hashes-rpc LMHASH:NTHASH stringbinding` : authenticate using NTLM hashes for RPC
- `rpcmap.py -debug stringbinding` : enable debug output for verbose information

## Categories
network, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-rpcmap
