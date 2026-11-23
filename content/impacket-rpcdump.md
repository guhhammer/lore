# impacket-rpcdump

This command is an alias of `rpcdump.py`.

## Usage
- `rpcdump.py domain/username:password@target` : dump RPC endpoints using username and password
- `rpcdump.py -hashes LMHASH:NTHASH domain/username:password@target` : dump RPC endpoints using NTLM hashes
- `rpcdump.py -target-ip target_ip domain/username:password@target` : specify a target IP address explicitly (useful if the target name is a NetBIOS name)
- `rpcdump.py -port port_number domain/username:password@target` : connect to a specific port (default is 135 for RPC Endpoint Mapper)
- `rpcdump.py -debug domain/username:password@target` : enable debug output

## Categories
users, network, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-rpcdump
