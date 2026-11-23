# alias

Create aliases - words that are replaced by a command string. Aliases expire with the current shell session unless defined in the shell's configuration file, e.g. `~/.bashrc` for Bash or `~/.zshrc` for Zsh.

## Usage
- `alias` : list all aliases
- `alias word="command"` : create a generic alias
- `alias word` : view the command associated to a given alias
- `unalias word` : remove an aliased command
- `alias rm="rm --interactive"` : turn `rm` into an interactive command
- `alias la="ls --all"` : create `la` as a shortcut for `ls --all`

## See also
- unalias

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
- tldr page: https://tldr.inbrowser.app/pages/common/alias
