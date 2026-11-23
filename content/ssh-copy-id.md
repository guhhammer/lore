# ssh-copy-id

Install your public key in a remote machine's authorized_keys.

## Usage
- `ssh-copy-id username@remote_host` : copy your keys to the remote machine
- `ssh-copy-id -i path/to/certificate username@remote_host` : copy the given public key to the remote
- `ssh-copy-id -i path/to/certificate -p port username@remote_host` : copy the given public key to the remote with specific port

## Categories
files, network, packages

## Links
- More information: https://manned.org/ssh-copy-id
- tldr page: https://tldr.inbrowser.app/pages/common/ssh-copy-id
