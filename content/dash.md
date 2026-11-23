# dash

Debian Almquist Shell, a modern, POSIX-compliant implementation of `sh` (not Bash-compatible).

## Usage
- `dash` : start an interactive shell session
- `dash -c "echo 'dash is executed'"` : execute specific commands
- `dash path/to/script.sh` : execute a specific script
- `dash -n path/to/script.sh` : check a specific script for syntax errors
- `dash -x path/to/script.sh` : execute a specific script while printing each command before executing it
- `dash -e path/to/script.sh` : execute a specific script and stop at the first error
- `echo "echo 'dash is executed'" | dash` : execute specific commands from `stdin`

## Categories
development, files, shell

## Links
- More information: https://manned.org/dash
- tldr page: https://tldr.inbrowser.app/pages/common/dash
