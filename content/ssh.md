# ssh

Opens a shell on another machine over the network, encrypted, or runs one
command there.

## Usage
- `ssh user@host` : log in to host as user
- `ssh -p 2222 user@host` : on another port
- `ssh user@host uptime` : run one command there and come back
- `ssh -t user@host lore` : run a full-screen program there (like lore)
- `ssh-keygen -t ed25519` : make a key pair, for logging in without a password
- `ssh-copy-id user@host` : put your key on the host

## Notes
The first time you connect, ssh shows the host's fingerprint; compare it
with the one the host's owner gives you before you answer yes.

## Categories
network, security

## Links
- OpenSSH manual: https://man.openbsd.org/ssh
