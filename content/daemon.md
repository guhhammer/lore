# daemon

Turn other processes into daemons.

## Usage
- `daemon -n "name" command` : run a command as a daemon
- `daemon -n "name" -r command` : run a command as a daemon which will restart if the command crashes
- `daemon -n "name" -r -A 2 -L 10 command` : run a command as a daemon which will restart if it crashes, with two attempts every 10 seconds
- `daemon -n "name" -l path/to/file.log command` : run a command as a daemon, writing logs to a specific file
- `daemon -n "name" --stop` : kill a daemon (`SIGTERM`)
- `daemon --list` : list daemons

## Categories
system, files, processes

## Links
- More information: https://manned.org/daemon.1
- tldr page: https://tldr.inbrowser.app/pages/linux/daemon
