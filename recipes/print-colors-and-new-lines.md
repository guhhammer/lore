# print colors and new lines

Print text in color, and print new lines from a string, which plain echo may not do.

## Steps
- `printf 'one\ntwo\n'` : new lines, the same in every shell
- `echo -e 'one\ntwo'` : the same with bash's echo
- `printf '\033[31mred\033[0m\n'` : red text, then back to normal
- `printf '\033[1;32mOK\033[0m\n'` : bold green
- `echo "$(tput setaf 4)blue$(tput sgr0)"` : colors by name of the terminal's capability

## Notes
- Colors 30 to 37: black, red, green, yellow, blue, magenta, cyan, white; 1; makes them bold.

## See also
- printf
- tput

## Categories
shell
