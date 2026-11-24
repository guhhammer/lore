# shell scripting

The basics of writing bash scripts: variables, tests, loops, functions, arguments and exit codes. Examples are for bash; most work in zsh too.

## Tasks
### Start safely
- `#!/usr/bin/env bash` : first line of a script, picks bash
- `set -euo pipefail` : stop on errors, unset variables and failed pipes
- `chmod +x script.sh` : make the script runnable
- `./script.sh` : run it from the current folder
- `bash -x script.sh` : run it printing every command, to debug
- `shellcheck script.sh` : find common mistakes
### Variables
- `NAME="value"` : set a variable, no spaces around the equals sign
- `echo "$NAME"` : use it, always in quotes
- `echo "${NAME:-default}"` : use a default when it is empty or unset
- `TODAY=$(date +%F)` : keep a command's output in a variable
- `export NAME` : pass it on to programs you start
- `read -r -p "Name: " NAME` : ask the user
### Arguments
- `echo "$1" "$2"` : the first and second argument
- `echo "$@"` : all arguments, each kept whole
- `shift` : drop the first argument
### Tests
- `if [ -f FILE ]; then echo yes; fi` : test that a file exists
- `if [[ "$A" == "x" ]]; then echo same; fi` : compare strings
- `if [ "$N" -gt 5 ]; then echo big; fi` : compare numbers
- `if [ -z "$NAME" ]; then echo empty; fi` : test for an empty value
- `if command -v git >/dev/null; then echo found; fi` : test that a program exists
### Loops
- `for f in *.txt; do echo "$f"; done` : loop over files
- `for i in {1..5}; do echo "$i"; done` : count
- `while read -r line; do echo "$line"; done < FILE` : loop over the lines of a file
- `case "$1" in start) echo go ;; stop) echo halt ;; esac` : choose by value
### Functions and exit codes
- `greet() { echo "hello $1"; }` : define a function
- `greet Ana` : call it
- `echo $?` : the exit code of the last command (0 is success)
- `exit 1` : leave the script with an error
- `cmd || echo failed` : run something only when cmd fails
- `trap 'rm -f "$TMP"' EXIT` : clean up when the script ends

## Notes
- Quote variables: `"$NAME"`, or spaces in a value split it into several words.
- `[ ]` needs spaces inside the brackets.
- `set -e` has odd corners (conditions, pipes): do not rely on it for everything.
- In a script `sh` may be a smaller shell than bash: use the bash shebang for bash features.

## See also
- bash
- shellcheck
- awk
- sed

## Links
- Bash manual: https://www.gnu.org/software/bash/manual/bash.html
- ShellCheck: https://www.shellcheck.net

## Categories
shell development
