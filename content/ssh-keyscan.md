# ssh-keyscan

Get the public SSH keys of remote hosts.

## Usage
- `ssh-keyscan hostname` : retrieve all public SSH keys of a remote host
- `ssh-keyscan -p port hostname` : retrieve all public SSH keys of a remote host listening on a specific port
- `ssh-keyscan -t rsa,dsa,ecdsa,ed25519 hostname` : retrieve certain types of public SSH keys of a remote host
- `ssh-keyscan -H hostname >>~/.ssh/known_hosts` : manually update the SSH known_hosts file with the fingerprint of a given host

## Categories
network

## Links
- More information: https://man.openbsd.org/ssh-keyscan
- tldr page: https://tldr.inbrowser.app/pages/common/ssh-keyscan
