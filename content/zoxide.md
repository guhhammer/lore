# zoxide

Keep track of the most frequently used directories. Uses a ranking algorithm to navigate to the best match.

## Usage
- `zoxide query string` : go to the highest-ranked directory that contains `string` in the name
- `zoxide query string1 string2` : go to the highest-ranked directory that contains `string1` and then `string2`
- `zoxide query -i` : start an interactive directory search (requires `fzf`)
- `zoxide add path/to/directory` : add a directory or increment its rank
- `zoxide remove path/to/directory` : remove a directory from `zoxide`'s database
- `zoxide init bash|elvish|fish|nushell|posix|powershell|tcsh|xonsh|zsh` : generate shell configuration for command aliases (`z`, `zi`)

## Categories
files, databases

## Links
- More information: https://manned.org/zoxide
- tldr page: https://tldr.inbrowser.app/pages/common/zoxide
