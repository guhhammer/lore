# set up an ssh key

Make a key, put it on a server to log in without a password, use a given key, change its passphrase.

## Steps
- `ssh-keygen -t ed25519` : make a key pair in ~/.ssh; the passphrase protects it
- `ssh-copy-id NAME@host` : put the public key on the server
- `ssh -i ~/.ssh/work_key NAME@host` : log in with a given key
- `ssh-keygen -p -f ~/.ssh/id_ed25519` : change or remove its passphrase
- `GIT_SSH_COMMAND='ssh -i ~/.ssh/work_key' git clone git@github.com:org/repo.git` : use a given key for git
- `chmod 600 ~/.ssh/id_ed25519` : "permissions too open": only you may read a private key

## Notes
- Hosts and keys can go in ~/.ssh/config: Host work, HostName, User, IdentityFile.
- Share only the .pub file; the other one stays on your machine.

## See also
- ssh-keygen
- ssh-copy-id
- ssh

## Categories
network security
