# print.zsh

Z Shell (`zsh`) builtin. Print arguments, similar to `echo`.

## Usage
- `print "Hello" "World"` : print input
- `print -l "Line1" "Line 2" "Line3"` : print separated by newline(s)
- `print -n "Hello"; print "World"` : print without trailing newline
- `print -e "Line 1\nLine2"` : enable backslash escapes
- `print -f "%s is %d years old.\n" "Alice" 30` : print arguments as described by `printf` (for greater portability across shells, consider using the `printf` command instead)

## See also
- echo
- printf
- zsh

## Categories
documents, shell

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/print.zsh
