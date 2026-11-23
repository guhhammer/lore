# fish

The Friendly Interactive SHell, a command-line interpreter designed to be user friendly.

## Usage
- `fish` : start an interactive shell session
- `fish -N` : start an interactive shell session without loading startup configs
- `fish -c "echo 'fish is executed'"` : execute specific commands
- `fish path/to/script.fish` : execute a specific script
- `fish -n path/to/script.fish` : check a specific script for syntax errors
- `echo "echo 'fish is executed'" | fish` : execute specific commands from `stdin`
- `fish -P` : start an interactive shell session in private mode, where the shell does not access old history or save new history
- `set -U -x variable_name variable_value` : define and export an environmental variable that persists across shell restarts (builtin)

## Categories
shell, development, text

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html
- tldr page: https://tldr.inbrowser.app/pages/common/fish
