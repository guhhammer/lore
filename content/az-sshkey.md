# az sshkey

Manage SSH public keys with virtual machines. Part of `azure-cli` (also known as `az`).

## Usage
- `az sshkey create --name name -g resource_group` : create a new SSH key
- `az sshkey create --name name -g resource_group --public-key "@path/to/key.pub"` : upload an existing SSH key
- `az sshkey list` : list all SSH public keys
- `az sshkey show --name name -g resource_group` : show information about an SSH public key

## Categories
network, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/sshkey
- tldr page: https://tldr.inbrowser.app/pages/common/az-sshkey
