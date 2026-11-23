# ssh-keygen

Generate SSH keys used for authentication, password-less logins, and other things.

## Usage
- `ssh-keygen` : generate a key interactively
- `ssh-keygen -f ~/.ssh/filename -t ed25519 -a 32` : generate an ed25519 key with 32 key derivation function rounds and save the key to a specific file
- `ssh-keygen -t rsa -b 4096 -C "comment|email"` : generate an RSA 4096-bit key with email as a comment
- `ssh-keygen -R remote_host` : remove the keys of a host from the `known_hosts` file (useful when a known host has a new key)
- `ssh-keygen -f ~/.ssh/filename -l -E md5` : retrieve the fingerprint of a key in MD5 Hex
- `ssh-keygen -f ~/.ssh/filename -p` : change the password of a key
- `ssh-keygen -f ~/.ssh/OpenSSH_private_key -p -m PEM` : change the type of the key format (for example from OPENSSH format to PEM), the file will be rewritten in-place
- `ssh-keygen -f ~/.ssh/OpenSSH_private_key -y` : retrieve public key from private key

## See also
- ssh-copy-id

## Categories
network, users, files

## Links
- More information: https://man.openbsd.org/ssh-keygen
- tldr page: https://tldr.inbrowser.app/pages/common/ssh-keygen
