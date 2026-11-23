# rpcdump.py

Dump remote RPC endpoints information via the Endpoint Mapper. Part of the Impacket suite.

## Usage
- `rpcdump.py domain/username:password@target` : dump RPC endpoints using username and password
- `rpcdump.py -hashes LMHASH:NTHASH domain/username:password@target` : dump RPC endpoints using NTLM hashes
- `rpcdump.py -target-ip target_ip domain/username:password@target` : specify a target IP address explicitly (useful if the target name is a NetBIOS name)
- `rpcdump.py -port port_number domain/username:password@target` : connect to a specific port (default is 135 for RPC Endpoint Mapper)
- `rpcdump.py -debug domain/username:password@target` : enable debug output

## Categories
users, network

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/rpcdump.py
