# safe

Interact with HashiCorp Vault.

## Usage
- `safe target vault_addr target_name` : add a safe target
- `safe auth authentication_token` : authenticate the CLI client against the Vault server, using an authentication token
- `safe env` : print the environment variables describing the current target
- `safe tree path` : display a tree hierarchy of all reachable keys for a given path
- `safe move path/to/old_secret path/to/new_secret` : move a secret from one path to another
- `safe ssh 2048 path/to/secret` : generate a new 2048-bit SSH key-pair and store it
- `safe set path/to/secret key=value` : set non-sensitive keys for a secret
- `safe gen path/to/secret key` : set auto-generated password in a secret

## Categories
files, network

## Links
- More information: https://github.com/egen/safe
- tldr page: https://tldr.inbrowser.app/pages/common/safe
