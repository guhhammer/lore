# see each command a script runs

Debug a script: print every command as it runs, check its syntax, find common mistakes.

## Steps
- `bash -x script.sh` : run it, printing each command first
- `set -x` : inside the script: print from here on; set +x stops
- `bash -n script.sh` : check the syntax without running it
- `shellcheck script.sh` : the usual mistakes, with how to fix them (shellcheck)

## See also
- bash
- shellcheck

## Categories
shell development
