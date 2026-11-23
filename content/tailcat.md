# tailcat

WireGuard-encrypted tunnels using Tailscale's data plane, like `netcat` but encrypted. In the examples below, an `addrblob` is the connection token (`tcom...`) printed by a running `tailcat` server, used by clients to reach it.

## Usage
- `tailcat` : start a server that accepts one connection and writes to `stdout` (no local port; clients connect via the printed addrblob)
- `tailcat --serve=22,80,443` : start a server that exposes specific local TCP ports (e.g. an existing service on 22/80/443) to clients via the tunnel
- `tailcat --serve=exit-node` : start a server as an exit node, routing client traffic through this host's network
- `tailcat --serve=no-auth-ssh` : start an auth-free SSH server (Linux/macOS)
- `cat path/to/file | tailcat addrblob port` : connect to a server with an addrblob on a specific port, sending `stdin` to it
- `tailcat ping --until-direct addrblob` : ping a server to test connectivity, waiting for a direct path
- `tailcat ssh addrblob` : SSH into a remote machine (server must expose port 22)
- `tailcat genkey --force` : generate a new server key and print its addrblob

## Categories
network, files

## Links
- More information: https://github.com/tailscale/tailcat#usage
- tldr page: https://tldr.inbrowser.app/pages/common/tailcat
