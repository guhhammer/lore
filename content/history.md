# history

Manage command-line history.

## Usage
- `history` : display the commands history list with line numbers
- `history 20` : display the last 20 commands (in Zsh it displays all commands starting from the 20th)
- `history -d|f|i|E` : display history with timestamps in different formats (only available in Zsh)
- `history -c` : clear the commands history list
- `history -w` : overwrite history file with history of current Bash shell (often combined with `history -c` to purge history)
- `history -d offset` : delete the history entry at the specified offset
- `history -s command` : add a command to history without running it
- `<Space>command` : run a command without adding it to history by adding a leading space

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-history
- tldr page: https://tldr.inbrowser.app/pages/common/history
- Wikipedia: https://en.wikipedia.org/wiki/History_(command)
