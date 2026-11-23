# ssh-add

Manage loaded SSH keys in the `ssh-agent`. Ensure that `ssh-agent` is up and running for the keys to be loaded in it.

## Usage
- `ssh-add` : add the default SSH keys in `~/.ssh` to the ssh-agent
- `ssh-add path/to/private_key` : add a specific key to the `ssh-agent`
- `ssh-add -l` : list fingerprints of currently loaded keys
- `ssh-add -d path/to/private_key` : delete a key from the `ssh-agent`
- `ssh-add -D` : delete all currently loaded keys from the `ssh-agent`
- `ssh-add -K path/to/private_key` : add a key to the `ssh-agent` and the keychain

## Categories
network, files

## Links
- More information: https://man.openbsd.org/ssh-add
- tldr page: https://tldr.inbrowser.app/pages/common/ssh-add
