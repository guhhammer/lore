# set

Toggle shell options or set the values of positional parameters.

## Usage
- `set` : display the names and values of shell variables
- `set -a` : export newly initialized variables to child processes
- `set -b` : write formatted messages to `stderr` when jobs finish
- `set -o vi` : write and edit text in the command-line with `vi`-like keybindings (e.g. `yy`)
- `set -o emacs` : return to default (`emacs`) mode
- `set -o` : list all modes
- `set -e` : exit the shell when (some) command fails
- `set -- argument1 argument2 ...` : reset all shell parameters and assign new ones

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#The-Set-Builtin
- tldr page: https://tldr.inbrowser.app/pages/common/set
