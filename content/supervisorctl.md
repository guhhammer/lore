# supervisorctl

Client/server system that allows its users to control a number of processes on UNIX-like operating systems. Command-line client piece of the supervisor which provides a shell-like interface.

## Usage
- `supervisorctl status process_name` : show the status of a process (or all processes if `process_name` is not specified)
- `supervisorctl start|stop|restart process_name` : start/stop/restart a process
- `supervisorctl start|stop|restart group_name:*` : start/stop/restart all processes in a group
- `supervisorctl tail -100 process_name stderr` : show last 100 bytes of process `stderr`
- `supervisorctl tail -f process_name stdout` : keep displaying `stdout` of a process
- `supervisorctl update` : reload process configuration file to add/remove processes as necessary

## Categories
processes, network, users

## Links
- More information: https://supervisord.org/running.html#running-supervisorctl
- tldr page: https://tldr.inbrowser.app/pages/common/supervisorctl
