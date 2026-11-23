# complete

Get and set argument autocompletion rules of shell commands in Bash. The specified completions will be invoked when `<Tab>` is pressed in Bash.

## Usage
- `complete -F function command` : set arguments of a command to autocomplete through a function (completion response is sent in `$COMPREPLY` variable)
- `complete -C autocomplete_command command` : set arguments of a command to autocomplete through another command (`$1` is the command, `$2` is the argument the cursor is on, and `$3` is the argument preceding the cursor)
- `complete -b command` : set arguments of a command to autocomplete to shell builtins
- `complete -o nospace -F function command` : apply autocompletion without appending a space to the completed word
- `complete -p` : list all loaded complete specifications
- `complete -p command` : list loaded complete specifications for a command

## See also
- compgen
- compopt

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-complete
- tldr page: https://tldr.inbrowser.app/pages/common/complete
