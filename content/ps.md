# ps

Information about running processes.

## Usage
- `ps aux` : list all running processes
- `ps auxww` : list all running processes including the full command string
- `ps aux | grep [s]tring` : search for a process that matches a string (the brackets will prevent `grep` from matching itself)
- `ps -u $(id -u) -F` : list all processes of the current user in extra full format
- `ps -u $(id -u) f` : list all processes of the current user as a tree
- `ps -o ppid= -p process_id` : get the parent PID of a process
- `ps k size` : sort processes by memory consumption
- `ps -L -p process_id` : display threads for a specific process

## Categories
processes

## Links
- More information: https://manned.org/ps
- tldr page: https://tldr.inbrowser.app/pages/common/ps
- Wikipedia: https://en.wikipedia.org/wiki/Ps_(Unix)
