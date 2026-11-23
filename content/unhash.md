# unhash

Remove hashed executable locations.

## Usage
- `unhash command` : remove a specific command from the hash table
- `unhash -a alias` : unhash non-suffix aliases
- `unhash -s alias` : unhash suffix aliases
- `unhash -f function` : unhash shell functions
- `unhash -d directory` : unhash directories
- `unhash -f -m "pattern"` : unhash every function that matches a pattern

## See also
- hash

## Categories
shell

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/unhash
