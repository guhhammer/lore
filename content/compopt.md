# compopt

Print or change the completion options for a command. `-o` means enabled and `+o` means disabled.

## Usage
- `compopt command` : print the completion options for given command
- `compopt -o|+o option1 -o|+o option2 command` : enable or disable a completion option of a command
- `compopt` : print the options for the currently executing completion

## See also
- compgen
- complete

## Categories
documents, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-compopt
- tldr page: https://tldr.inbrowser.app/pages/common/compopt
