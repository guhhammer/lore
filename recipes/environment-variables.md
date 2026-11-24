# environment variables

List, read, set and remove environment variables, for a command or for the session.

## Steps
- `printenv | sort` : every variable
- `echo "$HOME"` : one of them
- `export NAME=value` : set one for this shell and what it starts
- `NAME=value command` : set one for one command only
- `unset NAME` : remove one
- `set -a; source .env; set +a` : load a file of NAME=value lines into the shell

## Notes
- To keep a variable, put the export line in ~/.bashrc (or ~/.zshrc).

## See also
- printenv
- export
- env

## Categories
shell
