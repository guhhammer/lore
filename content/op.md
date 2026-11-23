# op

Official CLI for 1Password's desktop app.

## Usage
- `op signin` : sign in to a 1Password account
- `op vault list` : list all vaults
- `op item get item_name --format json` : print item details in JSON format
- `op item create --category category_name` : create a new item with a category in the default vault
- `op read secret_reference` : print a referenced secret to `stdout`
- `op run -- command` : pass secret references from exported environment variables to a command
- `op run --env-file path/to/env_file.env -- command` : pass secret references from an environment file to a command
- `op inject --in-file path/to/input_file --out-file path/to/output_file` : read secret references from a file and save plaintext secrets to a file

## Categories
files, desktop

## Links
- More information: https://developer.1password.com/docs/cli/reference/
- tldr page: https://tldr.inbrowser.app/pages/common/op
