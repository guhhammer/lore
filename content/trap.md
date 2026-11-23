# trap

Execute a command upon an event.

## Usage
- `trap` : list the commands and the names of the expected events
- `trap 'echo "Caught signal SIGHUP"' HUP` : execute a command when a signal is received
- `trap - HUP INT` : remove commands

## Categories
shell

## Links
- More information: https://manned.org/trap.1posix
- tldr page: https://tldr.inbrowser.app/pages/common/trap
