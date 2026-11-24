# mosh

Mobile Shell (`mosh`) is a robust and responsive replacement for SSH. `mosh` persists connections to remote servers while roaming between networks.

## Usage
- `mosh username@remote_host` : connect to a remote server
- `mosh --ssh="ssh -i path/to/key_file" username@remote_host` : connect to a remote server with a specific identity (private key)
- `mosh --ssh="ssh -p 2222" username@remote_host` : connect to a remote server using a specific port
- `mosh remote_host -- command -with -flags` : run a command on a remote server
- `mosh -p 124 username@remote_host` : select Mosh UDP port (useful when `remote_host` is behind a NAT)
- `mosh --server=path/to/mosh-server remote_host` : connect to a remote server when the `mosh-server` binary is outside the standard path

## Categories
network, files, shell

## Links
- More information: https://manned.org/mosh
- tldr page: https://tldr.inbrowser.app/pages/common/mosh
- Wikipedia: https://en.wikipedia.org/wiki/Mosh_(software)
