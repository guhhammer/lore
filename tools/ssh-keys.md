# ssh keys

SSH keys let you log in to servers without typing a password: you keep a private key and put the public half on the server. This page covers making keys, copying them, the agent, the config file and tunnels.

## Tasks
### Make a key
- `ssh-keygen -t ed25519 -C "you@laptop"` : make a modern key pair; Enter keeps the default file
- `ssh-keygen -t rsa -b 4096` : an RSA key, for old servers
- `ssh-keygen -p -f ~/.ssh/KEY` : change or add a passphrase
- `ssh-keygen -y -f ~/.ssh/KEY` : print the public key from a private one
- `ssh-keygen -lf ~/.ssh/KEY.pub` : show a key's fingerprint
### Put it on a server
- `ssh-copy-id USER@HOST` : add your public key to the server
- `ssh-copy-id -i ~/.ssh/KEY.pub -p PORT USER@HOST` : choose the key and port
- `ssh-add -L` : print the public keys of the agent, to paste into GitHub or a panel
- `ssh -i ~/.ssh/KEY USER@HOST` : log in with a chosen key
- `ssh-keygen -R HOST` : forget a server's saved host key after it changed
### The agent
- `eval "$(ssh-agent -s)"` : start an agent in this shell
- `ssh-add ~/.ssh/KEY` : give the agent a key, so you type the passphrase once
- `ssh-add -l` : list the keys the agent holds
- `ssh-add -D` : remove every key from the agent
- `ssh -A USER@HOST` : forward the agent to the server (only to hosts you trust)
### The config file (~/.ssh/config)
- `Host NAME` : start a block; then HostName, User, Port and IdentityFile lines under it
- `ssh NAME` : log in with everything from the block
- `ProxyJump BASTION` : a line in a block to go through a jump host
- `ssh -J USER@BASTION USER@HOST` : jump through a host from the command line
### Tunnels
- `ssh -L 8080:localhost:80 USER@HOST` : reach the server's port 80 at localhost:8080
- `ssh -R 9000:localhost:3000 USER@HOST` : let the server reach your port 3000 at its 9000
- `ssh -D 1080 USER@HOST` : a SOCKS proxy on localhost:1080
- `ssh -N -f -L 5432:localhost:5432 USER@HOST` : a tunnel kept in the background
### Fix permissions
- `chmod 700 ~/.ssh && chmod 600 ~/.ssh/KEY ~/.ssh/authorized_keys` : permissions ssh insists on

## Notes
- Never share the private key (the file without `.pub`).
- Give the key a passphrase; the agent saves you retyping it.
- ssh ignores keys and `authorized_keys` that other users can read: fix the permissions first.
- `ssh -v HOST` shows why a login fails; use `-vvv` for more.
- A tunnel `-L` listens on your machine only unless you bind it wider on purpose.

## See also
- ssh
- scp
- rsync
- gpg

## Links
- ssh-keygen: https://man.openbsd.org/ssh-keygen
- ssh_config: https://man.openbsd.org/ssh_config

## Categories
security network
