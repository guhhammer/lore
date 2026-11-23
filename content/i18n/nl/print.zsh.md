# print.zsh

Z Shell (`zsh`) ingebouwd commando. Print argumenten, vergelijkbaar met `echo`.

## Usage
- `print "Hello" "World"` : print invoer
- `print -l "Line1" "Line 2" "Line3"` : print gescheiden door regeleinde(n)
- `print -n "Hello"; print "World"` : print zonder afsluitend regeleinde
- `print -e "Line 1\nLine2"` : schakel backslash-escapes in
- `print -f "%s is %d years old.\n" "Alice" 30` : print argumenten zoals beschreven door `printf` (overweeg voor betere overdraagbaarheid tussen shells om het `printf`-commando te gebruiken)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html
