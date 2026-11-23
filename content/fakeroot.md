# fakeroot

Run a command in an environment faking root privileges for file manipulation.

## Usage
- `fakeroot` : start the default shell as fakeroot
- `fakeroot -- command command_arguments` : run a command as fakeroot
- `fakeroot -s path/to/file -- command command_arguments` : run a command as fakeroot and save the environment to a file on exit
- `fakeroot -i path/to/file -- command command_arguments` : load a fakeroot environment and run a command as fakeroot
- `fakeroot -u -- command command_arguments` : run a command keeping the real ownership of files instead of pretending they are owned by root
- `fakeroot -h` : display help

## Categories
files

## Links
- More information: https://manned.org/fakeroot
- tldr page: https://tldr.inbrowser.app/pages/linux/fakeroot
