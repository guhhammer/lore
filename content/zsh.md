# zsh

Z SHell, a Bash-compatible command-line interpreter.

## Usage
- `zsh` : start an interactive shell session
- `zsh -c "echo 'zsh is executed'"` : execute specific commands
- `zsh path/to/script.zsh` : execute a specific script
- `zsh -n path/to/script.zsh` : check a specific script for syntax errors without executing it
- `echo "echo 'zsh is executed'" | zsh` : execute specific commands from `stdin`
- `zsh -x path/to/script.zsh` : execute a specific script, printing each command in the script before executing it
- `zsh -v` : start an interactive shell session in verbose mode, printing each command before executing it
- `zsh -f` : start Zsh without loading user level configuration (e.g. `~/.zshrc`)

## See also
- bash
- !
- ^

## Categories
development, shell, files

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Invocation.html#Invocation
- tldr page: https://tldr.inbrowser.app/pages/common/zsh
- Wikipedia: https://en.wikipedia.org/wiki/Z_shell
