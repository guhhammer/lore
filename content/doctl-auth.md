# doctl auth

Authenticate `doctl` with API tokens.

## Usage
- `doctl auth init --context token_label` : open a prompt to enter an API token and label its context
- `doctl auth ls` : list authentication contexts (API tokens)
- `doctl auth switch --context token_label` : switch contexts (API tokens)
- `doctl auth remove --context token_label` : remove a stored authentication context (API token)
- `doctl auth -h` : show available commands

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/auth/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-auth
