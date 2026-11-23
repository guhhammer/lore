# rpcinfo

Make an RPC call to an RPC server and reports what it finds.

## Usage
- `rpcinfo` : show full table of all RPC services registered on localhost
- `rpcinfo -s localhost` : show concise table of all RPC services registered on localhost
- `rpcinfo -m` : display table of statistics of rpcbind operations on localhost
- `rpcinfo -l remote_nfs_server_ip mountd 2` : display list of entries of given service name (mountd) and version number (2) on a remote nfs share
- `rpcinfo -d mountd 1` : delete the registration for version 1 of the mountd service for all transports

## Categories
system, databases

## Links
- More information: https://manned.org/rpcinfo
- tldr page: https://tldr.inbrowser.app/pages/linux/rpcinfo
