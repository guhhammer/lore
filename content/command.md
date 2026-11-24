# command

Force the shell to execute the program and ignore any functions, builtins, and aliases with the same name.

## Usage
- `command ls` : execute the `ls` program literally, even if an `ls` alias exists
- `command -p command_name` : find and execute a command using a default `$PATH` (`/bin:/usr/bin:/sbin:/usr/sbin:/etc:/usr/etc`) that guarantees to find all standard utilities

## Categories
shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-command
- tldr page: https://tldr.inbrowser.app/pages/common/command
- Wikipedia: https://en.wikipedia.org/wiki/Command_(Unix)
