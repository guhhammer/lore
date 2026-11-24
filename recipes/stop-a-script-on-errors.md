# stop a script on errors

Make a script stop at the first failing command, say where, and still allow the ones that may fail.

## Steps
- `set -euo pipefail` : at the top: stop on errors (-e), on unset variables (-u), on failing pipes
- `trap 'echo "failed at line $LINENO"' ERR` : say where it stopped
- `command || { echo 'command failed'; exit 1; }` : stop with a message of your own
- `command || true` : let this one fail
- `if command; then echo ok; fi` : test whether it worked: $? is its exit code

## See also
- set
- trap

## Categories
shell development
