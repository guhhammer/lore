# setpriv

Runs a program with chosen Linux privileges, capabilities and security settings, without needing a setuid helper.

## Usage
- `setpriv --reuid=1000 --regid=1000 --clear-groups id` : run a program as another user and group, dropping extra groups
- `setpriv --no-new-privs program` : run a program that can never gain privileges through setuid or file capabilities
- `setpriv --bounding-set=-all program` : remove every capability from the bounding set for the program
- `setpriv --dump` : show the current privilege state of the shell

## Notes
Changing the user needs root. Check the result with `--dump` before relying on it.

## Categories
users

## Links
- Documentation: https://www.kernel.org/pub/linux/utils/util-linux/
