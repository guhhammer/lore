# secrethub

Keep secrets out of configuration files.

## Usage
- `secrethub read path/to/secret` : print a secret to `stdout`
- `secrethub generate path/to/secret` : generate a random value and store it as a new or updated secret
- `secrethub write --clip path/to/secret` : store a value from the clipboard as a new or updated secret
- `echo "secret_value" | secrethub write path/to/secret` : store a value supplied on `stdin` as a new or updated secret
- `secrethub audit path/to/repo_or_secret` : audit a repository or secret

## Categories
files

## Links
- More information: https://github.com/secrethub/secrethub-cli
- tldr page: https://tldr.inbrowser.app/pages/common/secrethub
