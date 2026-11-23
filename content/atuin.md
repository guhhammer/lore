# atuin

Store your shell history in a searchable database. Optionally sync your encrypted history between machines.

## Usage
- `eval "$(atuin init bash|zsh|fish)"` : install atuin into your shell
- `atuin import auto` : import history from the shell default history file
- `atuin search command` : search shell history for a specific command
- `atuin register -u username -e email -p password` : register an account on the default sync server using the specified username, email, and password
- `atuin login -u username -p password` : log in to the default sync server
- `atuin sync` : sync history with the sync server

## Categories
shell, users, databases

## Links
- More information: https://docs.atuin.sh/
- tldr page: https://tldr.inbrowser.app/pages/common/atuin
