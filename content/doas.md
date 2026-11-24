# doas

Execute a command as another user.

## Usage
- `doas command` : run a command as root
- `doas -u user command` : run a command as another user
- `doas -s` : launch the default shell as root
- `doas -C path/to/config_file command` : parse a configuration file and check if the execution of a command as another user is allowed
- `doas -L` : make `doas` request a password even after it was supplied earlier

## See also
- sudo
- pkexec
- run0

## Categories
users

## Links
- More information: https://man.openbsd.org/doas
- tldr page: https://tldr.inbrowser.app/pages/common/doas
- Wikipedia: https://en.wikipedia.org/wiki/Doas
