# daemonize

Run a command (that does not daemonize itself) as a Unix daemon.

## Usage
- `daemonize command command_arguments` : run a command as a daemon
- `daemonize -p path/to/pidfile command command_arguments` : write the PID to the specified file
- `daemonize -l path/to/lockfile command command_arguments` : use a lock file to ensure that only one instance runs at a time
- `sudo daemonize -u user command command_arguments` : use the specified user account

## Categories
files, system, users

## Links
- More information: https://software.clapper.org/daemonize/
- tldr page: https://tldr.inbrowser.app/pages/linux/daemonize
