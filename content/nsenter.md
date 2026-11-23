# nsenter

Run a new command in a running process' namespace. Particularly useful for Docker images or chroot jails.

## Usage
- `nsenter -t process_id -a command command_arguments` : run a specific command using the same namespaces as an existing process
- `nsenter -t process_id --mount|uts|ipc|net|pid|user|cgroup command command_arguments` : run a specific command in an existing process's mount|UTS|IPC|network|PID|user|cgroup|time namespace
- `nsenter -t process_id -u -T -i -- command command_arguments` : run a specific command in an existing process's UTS, time, and IPC namespaces
- `nsenter -n/proc/process_id/ns/net -- command command_arguments` : run a specific command in an existing process's namespace by referencing procfs

## Categories
processes, containers, media

## Links
- More information: https://manned.org/nsenter
- tldr page: https://tldr.inbrowser.app/pages/linux/nsenter
