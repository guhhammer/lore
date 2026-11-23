# shopt

Manage Bash shell options: variables (stored in `$BASHOPTS`) that control behavior specific to the Bash shell. Generic POSIX shell variables (stored in `$SHELLOPTS`) are managed with the `set` command instead.

## Usage
- `shopt` : list all settable options and whether they are set
- `shopt -s option_name` : set an option
- `shopt -s extglob` : enable extended shell globbing
- `shopt -u option_name` : unset an option
- `shopt -p` : print a list of all options and their status formatted as runnable `shopt` commands
- `help shopt` : display help

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#The-Shopt-Builtin
- tldr page: https://tldr.inbrowser.app/pages/common/shopt
