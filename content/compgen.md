# compgen

Bash built-in command for generating possible completion matches in Bash. Usually used within custom completion functions.

## Usage
- `compgen -c` : list all shell built-ins, aliases, functions, and executables that you could run
- `compgen -V COMPREPLY -c str` : list all commands that you could run that start with a specified string and save results to `$COMPREPLY`
- `compgen -W "apple orange banana" a` : match against a wordlist
- `compgen -a` : list all aliases
- `compgen -A function` : list all functions that you could run
- `compgen -k` : show shell reserved keywords
- `compgen -ac ls` : see all available commands/aliases starting with `ls`
- `compgen -u` : list all users on the system

## See also
- complete
- compopt

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-compgen
- tldr page: https://tldr.inbrowser.app/pages/common/compgen
